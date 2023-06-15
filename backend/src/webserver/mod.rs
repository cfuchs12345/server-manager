mod appdata;
mod routes;

use actix_identity::IdentityMiddleware;
use actix_session::{storage::CookieSessionStore, SessionMiddleware};
use actix_web_httpauth::{
    extractors::{bearer::BearerAuth, AuthenticationError},
    headers::www_authenticate::bearer::Bearer,
    middleware::HttpAuthentication,
};
pub use appdata::AppData;

use actix_files as fs;
use actix_web::{
    cookie::Key, dev::ServiceRequest, middleware::Logger, web, App, HttpRequest, HttpServer, Result,
};
use std::{path::PathBuf, time::Duration};

use crate::{
    datastore::{self},
    models::error::AppError,
};

const AUTO_LOGOUT_AFTER_MINUTES: u64 = 30;

pub async fn start_webserver(
    bind_address: String,
    app_data: appdata::AppData,
) -> Result<(), AppError> {
    let secret_key = datastore::get_config()?.get_string("session_secret_key")?;

    HttpServer::new(move || {
        App::new()
            // Install the identity framework first.
            .wrap(
                IdentityMiddleware::builder()
                    .login_deadline(Some(Duration::from_secs(60 * AUTO_LOGOUT_AFTER_MINUTES)))
                    .build(),
            )
            // The identity system is built on top of sessions. You must install the session
            // middleware to leverage `actix-identity`. The session middleware must be mounted
            // AFTER the identity middleware: `actix-web` invokes middleware in the OPPOSITE
            // order of registration when it receives an incoming request.
            .wrap(
                SessionMiddleware::builder(
                    CookieSessionStore::default(),
                    Key::from(secret_key.as_bytes()),
                )
                .cookie_secure(false)
                .build(),
            )
            .app_data(web::Data::new(app_data.clone()))
            .wrap(Logger::default())
            .service(
                web::scope("/backend")
                    .configure(init_token_secured_api)
                    .wrap(HttpAuthentication::bearer(validator_fn)),
            )
            .service(web::scope("/backend_nt").configure(init_no_token_api))
            .configure(init_static)
    })
    .bind(bind_address)?
    .run()
    .await
    .map_err(AppError::from)
}

async fn validator_fn(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (actix_web::Error, ServiceRequest)> {
    let token = credentials.token();

    match datastore::is_valid_token(token) {
        Ok(valid) => {
            if valid {
                Ok(req)
            } else {
                log::warn!("Token is invalid");
                Err((AuthenticationError::new(Bearer::default()).into(), req))
            }
        }
        Err(err) => {
            log::error!("Error while validating token: {}", err);
            Err((AuthenticationError::new(Bearer::default()).into(), req))
        }
    }
}

fn init_no_token_api(cfg: &mut web::ServiceConfig) {
    cfg.service(routes::get_one_time_key);
    cfg.service(routes::authenticate);
    cfg.service(routes::get_users_exist);
    cfg.service(routes::post_first_user);
}

fn init_token_secured_api(cfg: &mut web::ServiceConfig) {
    cfg.service(routes::post_networks_action);

    cfg.service(routes::post_servers);
    cfg.service(routes::get_servers);
    cfg.service(routes::get_servers_by_ipaddress);
    cfg.service(routes::put_servers_by_ipaddress);
    cfg.service(routes::delete_servers_by_ipaddress);

    cfg.service(routes::post_servers_by_ipaddress_action);
    cfg.service(routes::post_servers_actions);

    cfg.service(routes::get_plugins);
    cfg.service(routes::get_plugins_actions);
    cfg.service(routes::put_plugins_actions);

    cfg.service(routes::post_dnsservers);
    cfg.service(routes::get_dnsservers);
    cfg.service(routes::delete_dnsservers);

    cfg.service(routes::get_system_dnsservers);
    cfg.service(routes::get_system_information);
    cfg.service(routes::get_smtp_config_valid);

    cfg.service(routes::get_users);
    cfg.service(routes::post_user);
    cfg.service(routes::delete_user);
    cfg.service(routes::put_user_changepassword);

    cfg.service(routes::get_monitoring_data);
    cfg.service(routes::get_monitoring_ids);

    cfg.service(routes::get_notifications);

    cfg.service(routes::get_config);
    cfg.service(routes::post_config);
}

fn init_static(cfg: &mut web::ServiceConfig) {
    // Files for frontend ==>
    cfg.route("/favicon.ico", web::get().to(fav_icon));
    cfg.route("/", web::get().to(index_html));
    cfg.route("/index.html", web::get().to(index_html));
    cfg.route("/{filename:main.*\\.js}", web::get().to(named_file));
    cfg.route("/{filename:polyfills.*\\.js}", web::get().to(named_file));
    cfg.route("/{filename:runtime.*\\.js}", web::get().to(named_file));
    cfg.route("/{filename:styles.*}", web::get().to(named_file));
    cfg.route(
        "/{filename:3rdpartylicenses.txt}",
        web::get().to(named_file),
    );

    cfg.route(
        "/assets/svg/{filename:.*\\.svg}",
        web::get().to(named_file_svg),
    );
    // <== files for frontend
}

async fn fav_icon(_req: HttpRequest) -> Result<fs::NamedFile, AppError> {
    handle_named_file("./server/static/images/favicon.ico")
}

async fn index_html(_req: HttpRequest) -> Result<fs::NamedFile, AppError> {
    handle_named_file("./server/static/index.html")
}

async fn named_file(req: HttpRequest) -> Result<fs::NamedFile, AppError> {
    handle_named_dir_and_filename("./server/static/", get_filename_from_request(req)?)
}

async fn named_file_svg(req: HttpRequest) -> Result<fs::NamedFile, AppError> {
    handle_named_dir_and_filename(
        "./server/static/assets/svg/",
        get_filename_from_request(req)?,
    )
}

fn get_filename_from_request(req: HttpRequest) -> Result<String, AppError> {
    let path: PathBuf =
        req.match_info().query("filename").parse().map_err(|err| {
            AppError::Unknown(format!("Could not parse path. Error was: {}", err))
        })?;

    Ok(path
        .as_os_str()
        .to_str()
        .ok_or(AppError::Unknown(format!(
            "Could not get named file {:?}",
            path
        )))?
        .to_owned())
}

fn handle_named_dir_and_filename(dir: &str, file: String) -> Result<fs::NamedFile, AppError> {
    handle_named_file(format!("{}{}", dir, file).as_str())
}

fn handle_named_file(file: &str) -> Result<fs::NamedFile, AppError> {
    let path_found = fs::NamedFile::open(file);
    match path_found {
        Ok(file) => Ok(file),
        Err(err) => {
            log::error!("File not found {}. Error was: {}", file, err);
            Err(AppError::Unknown(format!("File {} not found", file)))
        }
    }
}
