use std::path::Path;
use std::time::Duration;

use http::StatusCode;

use crate::datastore;
use crate::models::error::AppError;

#[cfg(all(target_os = "linux"))]
use std::io::prelude::*;
#[cfg(all(target_os = "linux"))]
use std::os::unix::net::UnixStream;

pub const GET: &str = "get";
pub const POST: &str = "post";
pub const PUT: &str = "put";

#[allow(dead_code)]
pub const DELETE: &str = "delete";

#[cfg(all(target_os = "linux"))]
const SOCKET_HTTP_POSTFIX: &str = " HTTP/1.1\r\nHost:localhost\r\n\r\n";

#[cfg(all(target_os = "linux"))]
pub async fn execute_socket_request(
    socket: &str,
    url: &str,
    method: &str,
    headers: Option<Vec<(String, String)>>,
    body: Option<String>,
) -> Result<String, AppError> {
    let header_map: http::HeaderMap = headers_to_map(headers);

    log::debug!("executing http request {} on {}", method, url);

    let socket_path = Path::new(socket);

    let mut unix_stream = UnixStream::connect(socket_path).expect("Could not create stream");

    let request_str = match method {
        GET | POST | DELETE | PUT => {
            format!("{} {} {}", method.to_uppercase(), url, SOCKET_HTTP_POSTFIX)
        }
        _ => format!("{} {} {}", GET.to_uppercase(), url, SOCKET_HTTP_POSTFIX), // default is also GET
    };

    write_request_and_shutdown(&mut unix_stream, request_str, header_map, body)?;
    read_from_stream(&mut unix_stream)
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
/// * ` AppError`if the given metho is invalid
///
pub async fn execute_http_request(
    url: &str,
    method: &str,
    headers: Option<Vec<(String, String)>>,
    body: Option<String>,
) -> Result<String, AppError> {
    let client = create_http_client();

    let header_map: http::HeaderMap = headers_to_map(headers);

    log::debug!("executing http request {} on {}", method, url);

    let response = match method {
        POST => {
            client
                .post(url)
                .headers(header_map)
                .body(body.unwrap_or("".to_string()))
                .send()
                .await
        }
        PUT => {
            client
                .put(url)
                .headers(header_map)
                .body(body.unwrap_or("".to_string()))
                .send()
                .await
        }
        GET => client.get(url).headers(header_map).send().await,
        _ => client.get(url).headers(header_map).send().await, // default is also a get
    };

    match response {
        Ok(res) => match res.status() {
            StatusCode::ACCEPTED => Ok(res.text().await.unwrap_or("".to_string())),
            StatusCode::OK => Ok(res.text().await.unwrap_or("".to_string())),
            y => {
                log::info!("Returned StatusCode was not ACCEPTED or OK but {:?}", y);
                Ok("".to_string())
            }
        },
        Err(err) => Err(AppError::from(err)),
    }
}

fn create_http_client() -> reqwest::Client {
    let config = datastore::get_config();
    let accept_self_signed_certificates =
        config.get_bool("accept_self_signed_certificates").unwrap();

    log::debug!(
        "accept self-signed certificates setting is {}",
        accept_self_signed_certificates
    );

    reqwest::Client::builder()
        .danger_accept_invalid_certs(accept_self_signed_certificates)
        .timeout(Duration::from_secs(1))
        .build()
        .unwrap()
}

#[cfg(all(target_os = "linux"))]
fn write_request_and_shutdown(
    unix_stream: &mut UnixStream,
    request: String,
    header_map: http::HeaderMap,
    body: Option<String>,
) -> Result<(), AppError> {
    let mut message = request;

    header_map.iter().for_each(|h| {
        message.push_str(format!("{}:{}", h.0, h.1.to_str().unwrap_or_default()).as_str())
    });

    if let Some(body) = body {
        message.push('\n');
        message.push_str(body.as_str());
    }

    log::debug!("sending message {}", message);

    unix_stream.write_all(message.as_bytes())?;

    unix_stream.shutdown(std::net::Shutdown::Write)?;

    Ok(())
}

#[cfg(all(target_os = "linux"))]
fn read_from_stream(unix_stream: &mut UnixStream) -> Result<String, AppError> {
    let mut response = String::new();
    unix_stream.read_to_string(&mut response)?;

    log::debug!(
        "We received this response from the unix stream: {}",
        response
    );
    Ok(response)
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
