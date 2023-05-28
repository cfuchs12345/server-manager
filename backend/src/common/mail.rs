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
        .from(from.parse().unwrap())
        .to(to.parse().unwrap())
        .subject(subject)
        .header(ContentType::TEXT_PLAIN)
        .body(String::from(body))
        .unwrap();

    let config = datastore::get_config();

    let smtp_username = config.get_string("smtp_username");
    let smtp_password = config.get_string("smtp_password");
    let smtp_host = config.get_string("smtp_host");

    if smtp_username.is_err() || smtp_password.is_err() || smtp_host.is_err() {
        log::error!(
            "config for smtp_username, smntp_password or smtp_host is invalid. Cannot send email"
        );
        return Err(AppError::EmailConfigError("smtp_...".to_string()));
    }

    let creds = Credentials::new(smtp_username.unwrap(), smtp_password.unwrap());

    let mailer = SmtpTransport::relay(smtp_host.unwrap().as_str())
        .unwrap()
        .credentials(creds)
        .build();

    match mailer.send(&email) {
        Ok(_) => println!("Email sent successfully!"),
        Err(err) => return Err(AppError::Unknown(format!("{}", err))),
    }

    Ok(true)
}

pub fn is_smtp_config_valid() -> bool {
    let config = datastore::get_config();
    let email_from = config.get_string("email_from");
    let smtp_username = config.get_string("smtp_username");
    let smntp_password = config.get_string("smtp_password");
    let smtp_host = config.get_string("smtp_host");

    email_from.is_ok() && smtp_username.is_ok() && smntp_password.is_ok() && smtp_host.is_ok()
}
