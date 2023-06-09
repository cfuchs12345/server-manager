use std::time::Duration;

use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

use crate::datastore;
use crate::models::error::AppError;

pub async fn send_email(from: &str, to: &str, subject: &str, body: &str) -> Result<bool, AppError> {
    if from.is_empty() {
        return Err(AppError::EmailConfigError("email_from".to_string()));
    }
    let email = Message::builder()
        .from(from.parse().map_err(AppError::from)?)
        .to(to.parse().map_err(AppError::from)?)
        .subject(subject)
        .header(ContentType::TEXT_PLAIN)
        .body(String::from(body))?;

    let config = datastore::get_config()?;

    let smtp_username = config.get_string("smtp_username")?;
    let smtp_password = config.get_string("smtp_password")?;
    let smtp_host = config.get_string("smtp_host")?;

    let creds = Credentials::new(smtp_username, smtp_password);

    let mailer = SmtpTransport::relay(smtp_host.as_str())?
        .timeout(Some(Duration::from_secs(5)))
        .credentials(creds)
        .build();

    let res = mailer.send(&email)?;

    log::debug!("mailer response: {:?}", res);

    println!("Email sent successfully!");

    Ok(true)
}

pub fn is_smtp_config_valid() -> Result<bool, AppError> {
    let config = datastore::get_config()?;
    let email_from = config.get_string("email_from");
    let smtp_username = config.get_string("smtp_username");
    let smntp_password = config.get_string("smtp_password");
    let smtp_host = config.get_string("smtp_host");

    Ok(email_from.is_ok() && smtp_username.is_ok() && smntp_password.is_ok() && smtp_host.is_ok())
}
