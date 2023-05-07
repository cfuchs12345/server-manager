use std::{time::Duration};

use http::StatusCode;

use crate::{inmemory};

pub const GET: &str = "get";
pub const POST: &str = "post";
pub const PUT: &str = "put";

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
            _ => Ok("".to_string())
        }
    },
    Err(err)=> Err(err)
    }
}


pub fn create_http_client() -> reqwest::Client {
    let config = inmemory::get_config();
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
