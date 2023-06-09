#[cfg(all(target_os = "linux"))]
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
const SOCKET_HTTP_POSTFIX: &str = "HTTP/1.1\r\nHost: 127.0.0.1\r\n\r\n";

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
    let client = create_http_client()?;

    let header_map: http::HeaderMap = headers_to_map(headers)?;

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

    log::debug!("response: {:?}", response);

    match response {
        Ok(res) => match res.status() {
            StatusCode::ACCEPTED => Ok(res.text().await.unwrap_or("".to_string())),
            StatusCode::OK => Ok(res.text().await.unwrap_or("".to_string())),
            y => {
                log::debug!("Returned StatusCode was not ACCEPTED or OK but {:?}", y);
                Err(AppError::NokOKResponse(
                    y,
                    res.text().await.unwrap_or_default(),
                ))
            }
        },
        Err(err) => Err(AppError::from(err)),
    }
}

fn create_http_client() -> Result<reqwest::Client, AppError> {
    let config = datastore::get_config()?;
    let accept_self_signed_certificates = config
        .get_bool("accept_self_signed_certificates")
        .unwrap_or(false);

    log::debug!(
        "accept self-signed certificates setting is {}",
        accept_self_signed_certificates
    );

    reqwest::Client::builder()
        .danger_accept_invalid_certs(accept_self_signed_certificates)
        .timeout(Duration::from_secs(1))
        .build()
        .map_err(AppError::from)
}

pub async fn execute_timeseries_db_query(query: &[(&str, &str)]) -> Result<String, AppError> {
    let config = datastore::get_timeseriesdb_config()?;

    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(1))
        .build()?;

    client
        .get(format!(
            "http://{}:{}/exec",
            config.get_host(),
            config.get_http_port()
        ))
        .query(query)
        .send()
        .await?
        .text()
        .await
        .map_err(AppError::from)
}

fn headers_to_map(headers_opt: Option<Vec<(String, String)>>) -> Result<http::HeaderMap, AppError> {
    let Some(headers) = headers_opt else {
        return Ok(http::HeaderMap::new());
    };

    let mut header_map: http::HeaderMap = http::HeaderMap::new();

    for header in headers {
        let left_side_of_tuple = &header.0;

        let res = left_side_of_tuple
            .split_once('=')
            .ok_or(AppError::InvalidArgument(
                "header is invalid. Contains no equals sign".to_owned(),
                Some(left_side_of_tuple.clone()),
            ))?;

        let name = http::header::HeaderName::from_lowercase(res.0.to_lowercase().as_bytes())?;
        let value = http::header::HeaderValue::from_str(res.1)?;

        header_map.insert(name, value);
    }

    Ok(header_map)
}

#[cfg(all(target_os = "linux"))]
pub async fn execute_socket_request(
    socket: &str,
    url: &str,
    method: &str,
    headers: Option<Vec<(String, String)>>,
    body: Option<String>,
) -> Result<String, AppError> {
    let header_map: http::HeaderMap = headers_to_map(headers)?;

    log::debug!("executing http request {} on {}", method, url);

    let socket_path = Path::new(socket);

    let mut unix_stream = UnixStream::connect(socket_path)?;

    let request_str = match method {
        GET | POST | DELETE | PUT => {
            format!("{} {} {}", method.to_uppercase(), url, SOCKET_HTTP_POSTFIX)
        }
        _ => format!("{} {} {}", GET.to_uppercase(), url, SOCKET_HTTP_POSTFIX), // default is also GET
    };

    write_request_and_shutdown(&mut unix_stream, request_str, header_map, body)?;
    read_from_stream(&mut unix_stream)
}

#[cfg(all(target_os = "linux"))]
fn write_request_and_shutdown(
    unix_stream: &mut UnixStream,
    request: String,
    _header_map: http::HeaderMap, // how to handle headers with sockets?
    body: Option<String>,
) -> Result<(), AppError> {
    let mut message = request;

    if let Some(body) = body {
        if !body.trim().is_empty() {
            message.push_str(body.as_str());
            message.push_str("\r\n");
        }
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
