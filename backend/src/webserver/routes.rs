use log_derive::logfn;
use std::net::IpAddr;
use std::vec;

use crate::common::{ClientKey, OneTimeKey};
use crate::models::config::dns_server::DNSServer;
use crate::models::config::Configuration;
use crate::models::error::AppError;
use crate::models::request::common::QueryParamsAsMap;
use crate::models::request::plugin::PluginsAction;
use crate::models::request::server::{
    NetworkActionType, NetworksAction, ServerAction, ServerActionType, ServersAction,
    ServersActionType,
};
use crate::models::request::user::PasswordChange;
use crate::models::response::status::Status;
use crate::models::response::system_information::SystemInformation;
use crate::models::server::Server;
use crate::models::token::UserToken;
use crate::models::users::User;
use crate::webserver::appdata::AppData;
use crate::{common, other_functions};
use crate::{datastore, other_functions::systeminfo, plugin_execution};
use actix_session::Session;
use actix_web::delete;
use actix_web::{get, post, put, web, HttpRequest, HttpResponse};
use http::{header, HeaderName, HeaderValue};
use sqlx::types::chrono::NaiveDateTime;

#[post("/networks/actions")]
pub async fn post_networks_action(
    data: web::Data<AppData>,
    query: web::Json<NetworksAction>,
) -> Result<HttpResponse, AppError> {
    let params_map = QueryParamsAsMap::from(query.params.clone());

    let dns_server_result = datastore::get_all_dnsservers(&data.app_data_persistence).await;

    match query.action_type {
        NetworkActionType::AutoDiscover => {
            let network = params_map
                .get("network")
                .ok_or(AppError::MissingURLParameter("network".to_owned()))?;
            let lookup_names: bool = params_map
                .get("lookup_names")
                .ok_or(AppError::MissingURLParameter("lookup_names".to_owned()))?
                .parse()?;

            if lookup_names
                && (dns_server_result.is_err()
                    || dns_server_result
                        .as_ref()
                        .expect("Could not get ref")
                        .is_empty())
            {
                Err(AppError::DNSServersNotConfigured())
            } else {
                let dns_servers = dns_server_result?;

                let list = plugin_execution::auto_discover_servers_in_network(
                    network,
                    lookup_names,
                    dns_servers,
                    &true,
                )
                .await?;

                Ok(HttpResponse::Ok().json(list))
            }
        }
    }
}

#[get("/servers")]
pub async fn get_servers(data: web::Data<AppData>) -> Result<HttpResponse, AppError> {
    let servers = datastore::get_all_servers(&data.app_data_persistence, true).await?;

    // client doesn't need to know the credentials and or parameters normally
    // only if a user wants to configure a feature, the information is required on the client side
    // reduces client memory, increases speed and makes the data more secure, if the server doesn't send it out
    let simplified_servers = datastore::simplify_servers_for_client(servers);

    Ok(HttpResponse::Ok().json(simplified_servers))
}

#[post("/servers")]
pub async fn post_servers(
    data: web::Data<AppData>,
    query: web::Json<Server>,
) -> Result<HttpResponse, AppError> {
    datastore::insert_server(&data.app_data_persistence, &query.0).await?;

    Ok(HttpResponse::Ok().finish())
}

#[post("/servers/actions")]
pub async fn post_servers_actions(
    data: web::Data<AppData>,
    query: web::Json<ServersAction>,
) -> Result<HttpResponse, AppError> {
    let params_map = QueryParamsAsMap::from(query.params.clone());

    match query.action_type {
        ServersActionType::Status => {
            let ips_to_check: Vec<IpAddr> = match params_map.get("ipaddresses") {
                Some(_list) => params_map
                    .get_split_by("ipaddresses", ",")
                    .ok_or(AppError::Unknown(
                        "Could not split ipaddresses by , ".to_owned(),
                    ))?
                    .iter()
                    .flat_map(|s| {
                        let ip: Result<IpAddr, _> = s.parse();
                        ip
                    })
                    .collect(),
                None => Vec::new(),
            };

            let list = other_functions::statuscheck::status_check(ips_to_check, true).await?;

            Ok(HttpResponse::Ok().json(list))
        }
        ServersActionType::FeatureScan => {
            let servers = datastore::get_all_servers(&data.app_data_persistence, true).await?;

            let upnp_activated = !datastore::is_plugin_disabled("upnp", &data.app_data_persistence)
                .await
                .unwrap_or(true);

            let list =
                plugin_execution::discover_features_of_all_servers(servers, upnp_activated, &true)
                    .await?;

            Ok(HttpResponse::Ok().json(list))
        }
        ServersActionType::ActionConditionCheck => {
            Ok(HttpResponse::Ok().json(datastore::get_all_condition_results()?.to_vec()))
        }
    }
}

#[post("/servers/{ipaddress}/actions")]
pub async fn post_servers_by_ipaddress_action(
    data: web::Data<AppData>,
    query: web::Json<ServerAction>,
    path: web::Path<String>,
) -> Result<HttpResponse, AppError> {
    let Ok(ipaddress): Result<IpAddr,_> = path.into_inner().parse() else {
        return Err(AppError::InvalidArgument("ipaddress".to_owned(), None));
    };

    let server = datastore::get_server(&data.app_data_persistence, &ipaddress).await?;

    let crypto_key = datastore::get_crypto_key()?;

    match query.action_type {
        ServerActionType::FeatureScan => {
            let list = plugin_execution::discover_features(ipaddress, crypto_key, &true).await?;

            Ok(HttpResponse::Ok().json(list))
        }
        ServerActionType::Status => {
            let list = other_functions::statuscheck::status_check(vec![ipaddress], false).await?;

            Ok(HttpResponse::Ok().json(list.first().unwrap_or(&Status {
                ipaddress,
                is_running: false,
            })))
        }
        ServerActionType::ExecuteFeatureAction => {
            let params_map = QueryParamsAsMap::from(query.params.clone());

            let feature_id = params_map
                .get("feature_id")
                .ok_or(AppError::ArgumentNotFound("feature_id".to_owned()))?;

            let action_id = params_map
                .get("action_id")
                .ok_or(AppError::ArgumentNotFound("action_id".to_owned()))?;

            let action_params = params_map.get_as_str("action_params").map(|v| v.to_owned()); // keep as option

            let feature = server
                .find_feature(feature_id)
                .ok_or(AppError::FeatureNotFound(
                    format!("{}", ipaddress),
                    feature_id.clone(),
                ))?;

            let result = plugin_execution::execute_action(
                &server,
                &feature,
                action_id,
                action_params,
                crypto_key,
                &false,
            )
            .await?;

            Ok(HttpResponse::Ok().json(result))
        }
        ServerActionType::QueryData => {
            let results = plugin_execution::execute_data_query(
                &server,
                &data.app_data_template_engine,
                crypto_key,
                &false,
            )
            .await?;

            Ok(HttpResponse::Ok().json(results))
        }
    }
}

#[put("/servers/{ipaddress}")]
pub async fn put_servers_by_ipaddress(
    data: web::Data<AppData>,
    query: web::Json<Server>,
) -> Result<HttpResponse, AppError> {
    datastore::update_server(&data.app_data_persistence, &query.0).await?;

    Ok(HttpResponse::Ok().finish())
}

#[get("/servers/{ipaddress}")]
#[logfn(err = "Error", fmt = "Could not get server: {:?}")]
pub async fn get_servers_by_ipaddress(
    session: Session,
    data: web::Data<AppData>,
    path: web::Path<String>,
    query: web::Query<std::collections::HashMap<String, String>>,
) -> Result<HttpResponse, AppError> {
    let ipaddress = path.into_inner().parse()?;
    let params = query.into_inner();

    let full_data: bool = params
        .get("full_data")
        .ok_or(AppError::MissingURLParameter(
            "expected parameter full_data but it is missing".to_owned(),
        ))?
        .parse()?;
    let server = datastore::get_server(&data.app_data_persistence, &ipaddress).await?;

    if full_data {
        let client_key = ClientKey::get_from_session(session)?.ok_or(AppError::Unknown(
            "Could not find client key in session".to_owned(),
        ))?;

        let re_encrypted_server =
            datastore::re_encrypt_server(server, client_key.key.as_str(), true)?;
        log::info!("full server: {:?}", re_encrypted_server);
        log::info!("key: {}", client_key.key);

        Ok(HttpResponse::Ok().json(re_encrypted_server))
    } else {
        let simplified_server = datastore::simplify_server_for_client(server);
        Ok(HttpResponse::Ok().json(simplified_server))
    }
}

#[delete("/servers/{ipaddress}")]
pub async fn delete_servers_by_ipaddress(
    data: web::Data<AppData>,
    path: web::Path<String>,
) -> Result<HttpResponse, AppError> {
    let ipaddress = path.into_inner().parse()?;

    datastore::delete_server(&data.app_data_persistence, &ipaddress).await?;

    Ok(HttpResponse::Ok().finish())
}

#[get("/plugins")]
pub async fn get_plugins(
    _data: web::Data<AppData>,
    _req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    Ok(HttpResponse::Ok().json(datastore::get_all_plugins()?))
}

#[get("/plugins/actions")]
pub async fn get_plugins_actions(
    data: web::Data<AppData>,
    query: web::Query<std::collections::HashMap<String, String>>,
) -> Result<HttpResponse, AppError> {
    let params = query.into_inner();

    let query_value = params
        .get("query")
        .ok_or(AppError::MissingURLParameter("query".to_owned()))?;

    match query_value.as_str() {
        "disabled" => {
            let persistence = &data.app_data_persistence;

            let list = datastore::get_disabled_plugins(persistence).await?;

            Ok(HttpResponse::Ok().json(list))
        }
        y => Err(AppError::UnsupportedURLParameter(
            "query".to_owned(),
            Some(y.to_owned()),
        )),
    }
}

#[put("/plugins/actions")]
pub async fn put_plugins_actions(
    data: web::Data<AppData>,
    query: web::Json<PluginsAction>,
) -> Result<HttpResponse, AppError> {
    let action = query.into_inner();
    let params_map = QueryParamsAsMap::from(action.params);

    let persistence = &data.app_data_persistence;

    let res = datastore::disable_plugins(
        persistence,
        params_map
            .get_split_by("ids", ",")
            .ok_or(AppError::UnsupportedURLParameter(
                "ids".to_owned(),
                params_map.get("ids").map(|v| v.to_owned()),
            ))?,
    )
    .await?;

    match res {
        true => Ok(HttpResponse::Ok().finish()),
        false => Err(AppError::Unknown("Could not disable plugin".to_owned())),
    }
}

#[post("/configurations/dnsservers")]
pub async fn post_dnsservers(
    data: web::Data<AppData>,
    query: web::Json<DNSServer>,
) -> Result<HttpResponse, AppError> {
    let persistence = &data.app_data_persistence;

    let server = query.into_inner();

    datastore::insert_dnsserver(persistence, &server).await?;
    Ok(HttpResponse::Ok().finish())
}

#[get("/systeminformation/dnsservers")]
pub async fn get_system_dnsservers(_data: web::Data<AppData>) -> Result<HttpResponse, AppError> {
    Ok(HttpResponse::Ok().json(systeminfo::get_systenms_dns_servers()?))
}

#[get("/configurations/dnsservers")]
pub async fn get_dnsservers(data: web::Data<AppData>) -> Result<HttpResponse, AppError> {
    let persistence = &data.app_data_persistence;

    let list = datastore::get_all_dnsservers(persistence).await?;
    Ok(HttpResponse::Ok().json(list))
}

#[delete("/configurations/dnsservers/{ipaddress}")]
pub async fn delete_dnsservers(
    data: web::Data<AppData>,
    path: web::Path<String>,
) -> Result<HttpResponse, AppError> {
    let persistence = &data.app_data_persistence;

    let ipaddress = path.into_inner().parse()?;

    datastore::delete_dnsserver(persistence, &ipaddress).await?;
    Ok(HttpResponse::Ok().finish())
}

#[get("/system/information")]
pub async fn get_system_information() -> Result<HttpResponse, AppError> {
    Ok(HttpResponse::Ok().json(SystemInformation {
        load_average: systeminfo::get_load_info(),
        memory_stats: systeminfo::get_memory_stats(),
        memory_usage: systeminfo::get_memory_usage(),
    }))
}

#[get("/system/smtpconfigvalid")]
pub async fn get_smtp_config_valid() -> Result<HttpResponse, AppError> {
    Ok(HttpResponse::Ok().json(common::is_smtp_config_valid()?))
}

#[get("/users")]
pub async fn get_users(data: web::Data<AppData>) -> Result<HttpResponse, AppError> {
    let result = datastore::get_all_users(&data.app_data_persistence).await?;

    Ok(HttpResponse::Ok().json(result))
}

#[get("/users/exist")]
pub async fn get_users_exist(data: web::Data<AppData>) -> Result<HttpResponse, AppError> {
    let result = datastore::get_all_users(&data.app_data_persistence).await?;

    Ok(HttpResponse::Ok().json(!result.is_empty()))
}

#[post("/users")]
pub async fn post_user(
    data: web::Data<AppData>,
    query: web::Json<User>,
) -> Result<HttpResponse, AppError> {
    save_user_common(data, query).await
}

#[post("/users_first")]
pub async fn post_first_user(
    data: web::Data<AppData>,
    query: web::Json<User>,
) -> Result<HttpResponse, AppError> {
    let result = datastore::get_all_users(&data.app_data_persistence).await?;

    if !result.is_empty() {
        log::error!("Called function that is used for initial user save that allows and update without authorization. However, there are already users. So this is not the initial user creation.");
        Ok(HttpResponse::Unauthorized().finish())
    } else {
        save_user_common(data, query).await
    }
}

async fn save_user_common(
    data: web::Data<AppData>,
    query: web::Json<User>,
) -> Result<HttpResponse, AppError> {
    let initial_password = common::generate_short_random_string();
    let password_hash = common::hash_password(initial_password.as_str())?;

    let mut user = query.0;
    user.update_password_hash(password_hash);

    let update_result = datastore::insert_user(&data.app_data_persistence, &user).await?;

    if update_result {
        if common::is_smtp_config_valid()? {
            let from_address = datastore::get_config()?.get_string("email_from")?;

            match common::send_email(from_address.as_str(),
                    &user.get_email(),
                    "Your initial password for the Server-Manager",
                    format!("Hello {},\n\nyour user id is '{}' and the initial password is: '{}'.\n\nRegards,\nyour Server-Manager", user.get_full_name(), user.get_user_id(), initial_password).as_str()).await {
                        Ok(res) => {
                            if res {
                                Ok(HttpResponse::Ok().finish()) // mail send successfully - no need to display the initual password
                            }
                            else {
                                Ok(HttpResponse::Ok().json(initial_password))
                            }
                        },
                        Err(err) => {
                            log::error!("An error occurred while trying to send the initial password mail: {}", err);
                            Ok(HttpResponse::Ok().json(initial_password))
                        }
                    }
        } else {
            Ok(HttpResponse::Ok().json(initial_password))
        }
    } else {
        Err(AppError::DatabaseError(format!(
            "Could not update user '{:?}'",
            user
        )))
    }
}

#[delete("/users/{user_id}")]
pub async fn delete_user(
    data: web::Data<AppData>,
    path: web::Path<String>,
) -> Result<HttpResponse, AppError> {
    let user_id = path.into_inner();

    datastore::delete_user(&data.app_data_persistence, &user_id).await?;

    Ok(HttpResponse::Ok().finish())
}

#[put("/user/{user_id}/changepassword")]
pub async fn put_user_changepassword(
    data: web::Data<AppData>,
    req: HttpRequest,
    query: web::Json<PasswordChange>,
) -> Result<HttpResponse, AppError> {
    let headers = req.headers();
    let custom_header = headers
        .get(HeaderName::from_static("x-custom"))
        .ok_or(AppError::Unknown("Missing header".to_owned()))?;

    let otk_tuple = get_existing_otk(custom_header).await?;

    let secret = common::make_aes_secrect(query.user_id.as_str(), otk_tuple.1.as_str());

    let decrypted_old = common::aes_decrypt(&query.old_password, secret.as_str())?;

    let mut user = datastore::get_user(&data.app_data_persistence, query.user_id.as_str()).await?;

    let password_check_result = user.check_password(&decrypted_old)?;

    if password_check_result {
        let decrypted_new = common::aes_decrypt(&query.new_password, secret.as_str())?;

        user.update_password_hash(common::hash_password(decrypted_new.as_str())?);

        let user_updated = datastore::update_user(&data.app_data_persistence, &user).await?;

        if user_updated {
            HttpResponse::Ok().finish();
        }
    } else {
        log::error!("Password check of old password failed");
    }

    Ok(HttpResponse::Unauthorized().finish())
}

#[get("users/authenticate/otk")]
pub async fn get_one_time_key() -> Result<HttpResponse, AppError> {
    let otk = OneTimeKey::generate().await?;

    Ok(HttpResponse::Ok().json(otk))
}

#[post("users/authenticate")]
pub async fn authenticate(
    session: Session,
    data: web::Data<AppData>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let headers = req.headers();
    let auth_header = headers
        .get(header::AUTHORIZATION)
        .ok_or(AppError::Unknown("Missing header value".to_owned()))?;
    let custom_header = headers
        .get(HeaderName::from_static("x-custom"))
        .ok_or(AppError::Unknown("Missing header value".to_owned()))?;

    let otk_tuple = get_existing_otk(custom_header).await?;

    let auth_values = get_auth_data_split(auth_header)?
        .ok_or(AppError::Unknown("Could not split header".to_owned()))?;

    let user_id = auth_values.0;
    let secret = common::make_aes_secrect(user_id.as_str(), otk_tuple.1.as_str());

    let decrypted = common::aes_decrypt(&auth_values.1, secret.as_str())?;

    let user = datastore::get_user(&data.app_data_persistence, user_id.as_str()).await?;

    let password_check_result = user.check_password(&decrypted)?;

    if password_check_result {
        let client_key = ClientKey::new().register_for_session(session)?;

        let token = common::generate_long_random_string();

        datastore::insert_token(&token)?;

        Ok(HttpResponse::Ok().json(UserToken {
            user_id,
            token,
            client_key: client_key.key,
        }))
    } else {
        Err(AppError::InvalidPassword())
    }
}

#[get("monitoring/ids")]
async fn get_monitoring_ids(
    data: web::Data<AppData>,
    query: web::Query<std::collections::HashMap<String, String>>,
) -> Result<HttpResponse, AppError> {
    let ipaddress_param = query
        .get("ipaddress")
        .ok_or(AppError::MissingURLParameter("ipaddress".to_owned()))?;

    let ipaddress = ipaddress_param.parse::<IpAddr>()?;
    let server = datastore::get_server(&data.app_data_persistence, &ipaddress).await?;

    let mut names: Vec<String> = Vec::new();

    for feature in &server.features {
        if let Some(plugin) = datastore::get_plugin(feature.id.as_str())? {
            let mut plugin_monitoring_ids: Vec<String> = plugin
                .data
                .iter()
                .flat_map(|d| d.monitoring.to_owned())
                .map(|m| m.id)
                .collect();

            names.append(&mut plugin_monitoring_ids);
        }
    }

    names.push("server_status".to_owned());

    Ok(HttpResponse::Ok().json(names))
}

#[get("monitoring/data")]
async fn get_monitoring_data(
    _data: web::Data<AppData>,
    query: web::Query<std::collections::HashMap<String, String>>,
) -> Result<HttpResponse, AppError> {
    //nm=true
    //query=SELECT timestamp, tempF FROM weather LIMIT 2;

    let ipaddress_param = query
        .get("ipaddress")
        .ok_or(AppError::MissingURLParameter("ipaddress".to_owned()))?;

    let series_id = query
        .get("series_id")
        .ok_or(AppError::MissingURLParameter("series_id".to_owned()))?;

    let ipaddress = ipaddress_param.parse::<IpAddr>()?;

    let response = plugin_execution::get_monitoring_data(series_id, ipaddress).await?;
    Ok(HttpResponse::Ok().json(response))
}

#[get("configuration")]
async fn get_config(data: web::Data<AppData>, req: HttpRequest) -> Result<HttpResponse, AppError> {
    let decrypted_password = get_decrypted_password_from_header(req).await?;

    Ok(HttpResponse::Ok().json(
        datastore::export_config(&data.app_data_persistence, decrypted_password.as_str()).await?,
    ))
}

#[post("configuration")]
async fn post_config(
    data: web::Data<AppData>,
    query: web::Json<Configuration>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let decrypted_password = get_decrypted_password_from_header(req).await?;

    let config = query.into_inner();

    Ok(HttpResponse::Ok().json(
        datastore::import_config(
            &data.app_data_persistence,
            config,
            true,
            &decrypted_password,
        )
        .await?,
    ))
}

async fn get_decrypted_password_from_header(req: HttpRequest) -> Result<String, AppError> {
    let headers = req.headers();

    let custom_header = headers
        .get(HeaderName::from_static("x-custom"))
        .ok_or(AppError::Unknown("Missing header".to_owned()))?;
    let custom_header2 = headers
        .get(HeaderName::from_static("x-custom2"))
        .ok_or(AppError::Unknown("Missing header".to_owned()))?;

    let encrypted_password: &str = custom_header2.to_str()?;

    let otk_tuple = get_existing_otk(custom_header).await?;
    let secret = common::make_aes_secrect("config", otk_tuple.1.as_str());

    common::aes_decrypt(encrypted_password, secret.as_str())
}

async fn get_existing_otk(header_value: &HeaderValue) -> Result<(NaiveDateTime, String), AppError> {
    let number: u32 = header_value.to_str()?.parse()?;

    common::OneTimeKey::get_one_time_key(number).await
}

fn get_auth_data_split(header_value: &HeaderValue) -> Result<Option<(String, String)>, AppError> {
    let val_str = header_value.to_str()?;
    let cut_val_str = val_str.replace("Basic ", "");

    let decoded = common::decode_base64_urlsafe_with_pad(cut_val_str.as_str())?;

    Ok(decoded
        .split_once(':')
        .map(|v| (v.0.to_owned(), v.1.to_owned())))
}
