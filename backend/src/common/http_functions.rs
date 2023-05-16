use std::time::Duration;

use http::StatusCode;

use crate::datastore;

#[cfg(all(target_os="linux"))]
use std::os::unix::net::UnixStream;

pub const GET: &str = "get";
pub const POST: &str = "post";
pub const PUT: &str = "put";
pub const DELETE: &str = "delete";

const SOCKET_HTTP_POSTFIX: &str = " HTTP/1.1\r\nHost:localhost\r\n\r\n";




#[cfg(all(target_os="linux"))]
pub async fn execute_socket_request(
    path: String,
    url: &str,
    method: &str,
    headers: Option<Vec<(String, String)>>,
    body: Option<String>
)  -> Result<String, reqwest::Error> {
    
    //#[cfg(all(target_os="linux"))]
    //http_functions::execute_socket_request( "/var/run/docker.sock".to_string(), "/containers/json", http_functions::GET, None, None).await.unwrap();
    
    let mut request_str = String::new();

    match method {
        GET | POST | DELETE | PUT => {
            request_str.push_str(method.to_uppercase().as_str());
            request_str.push_str(" ");
            request_str.push_str( url);
            request_str.push_str(SOCKET_HTTP_POSTFIX);
        },
        y => { // default is also GET
            request_str.push_str("GET");
            request_str.push_str(" ");
            request_str.push_str( url);
            request_str.push_str(SOCKET_HTTP_POSTFIX);            
        }
    }

    let socket_path = Path::new(path.as_str());
    
    let mut unix_stream =
    UnixStream::connect(&socket_path).expect("Could not create stream");
   
    write_request_and_shutdown(&mut unix_stream, request_str);
    Ok(read_from_stream(&mut unix_stream))
}

#[cfg(all(target_os="linux"))]
fn write_request_and_shutdown(unix_stream: &mut UnixStream, message: String) {
    log::debug!("sending message {}", message);

    unix_stream
        .write_all(message.as_bytes())
        .expect("Failed at writing onto the unix stream");

    unix_stream
        .shutdown(std::net::Shutdown::Write)
        .expect("Could not shutdown writing on the stream");
}

#[cfg(all(target_os="linux"))]
fn read_from_stream(unix_stream: &mut UnixStream) -> String {
    let mut response = String::new();
    unix_stream
        .read_to_string(&mut response)
        .expect("Failed at reading the unix stream");

        log::debug!("We received this response: {}", response);
    response
}




/// Executes an http request on the given url using the given method.
/// 
/// # Arguments
/// 
/// * `url` the url including protocol, port and so on
/// * `method` the http method to use (currently 'get', 'post' or 'put' are allowed)
/// * `header` optional of a vector of tuples with tuple (header_name, header_value)
/// * `body` and optional body to send for 'post' and 'put' requests
/// 
/// # Panics
/// never
/// * `
/// # Errors
/// * `request::error::Error` if the request fails during execution
/// * ` std::io::Error`if the given metho is invalid
///
pub async fn execute_http_request(
    url: String,
    method: &str,
    headers: Option<Vec<(String, String)>>,
    body: Option<String>
) -> Result<String, reqwest::Error> {
    
    let client = create_http_client();

    let header_map: http::HeaderMap = headers_to_map(headers);
    
    log::debug!("executing http request {} on {}", method, url);

    let response = match method {
        POST => client.post(url).headers(header_map).body(body.unwrap_or("".to_string())).send().await,
        PUT => client.put(url).headers(header_map).body(body.unwrap_or("".to_string())).send().await,
        GET => client.get(url).headers(header_map).send().await,
        _ => client.get(url).headers(header_map).send().await, // default is also a get
    };

    match response {
        Ok(res) => {
        match res.status() {
            StatusCode::ACCEPTED => Ok(res.text().await.unwrap_or("".to_string())),
            StatusCode::OK => Ok(res.text().await.unwrap_or("".to_string())),
            y => Ok(format!("Returned StatusCode was not ACCEPTED or OK but {:?}", y))
        }
    },
    Err(err)=> Err(err)
    }
}


pub fn create_http_client() -> reqwest::Client {
    let config = datastore::get_config();
    let accept_self_signed_certificates = config.get_bool("accept_self_signed_certificates").unwrap();

    log::debug!("accept self-signed certificates setting is {}", accept_self_signed_certificates);

    reqwest::Client::builder()
        .danger_accept_invalid_certs(accept_self_signed_certificates)
        .timeout(Duration::from_secs(1))
        .build()
        .unwrap()
}



fn headers_to_map(headers: Option<Vec<(String, String)>>) -> http::HeaderMap {
    if headers.is_none() {
        return http::HeaderMap::new();
    }
    let mut header_map: http::HeaderMap = http::HeaderMap::new();

    for header in headers.unwrap() {
        let res = header.0.split_once('=');

        if res.is_none() {
            log::error!(
                "Header {} is invalid. Container no equals sign (=)",
                header.1
            );
            continue;
        }
        let split = res.unwrap();

        let name_res = http::header::HeaderName::from_lowercase(split.0.to_lowercase().as_bytes());
        let value_res = http::header::HeaderValue::from_str(split.1);

        match name_res {
            Ok(name) => match value_res {
                Ok(value) => {
                    header_map.insert(name, value);
                }
                Err(err) => {
                    log::error!("Header {} is invalid {}", header.1, err);
                }
            },
            Err(err) => {
                log::error!("Header {} is invalid {}", header.1, err);
            }
        }
    }

    header_map
}

