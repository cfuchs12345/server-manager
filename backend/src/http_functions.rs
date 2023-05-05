use std::{time::Duration, io::ErrorKind};

use reqwest::Response;

pub const GET: &str = "get";
pub const POST: &str = "post";

pub async fn execute_http_request(
    url: String,
    method: &str,
    headers: Option<Vec<(String, String)>>,
    body: Option<String>,
    accept_self_signed_certificates: bool,
) -> Result<Response, Box<dyn std::error::Error>> {
    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(accept_self_signed_certificates)
        .timeout(Duration::from_secs(1))
        .build()
        .unwrap();

    let header_map: http::HeaderMap = headers_to_map(headers);

    let result = match method {
        GET => client.get(url).headers(header_map).send().await.map_err(|e| e.into()),
        POST => client.post(url).headers(header_map).body(body.unwrap_or("".to_string())).send().await.map_err(|e| e.into()),
        y => Err( std::io::Error::new(ErrorKind::InvalidInput, format!("Method {} is not supported here", y)).into())
    };
   
    result
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
