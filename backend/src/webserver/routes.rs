use std::net::IpAddr;

use crate::common::OneTimeKey;
use crate::models::config::dns_server::DNSServer;
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
use actix_web::delete;
use actix_web::{get, post, put, web, HttpRequest, HttpResponse};
use http::{header, HeaderName, HeaderValue};
use sqlx::types::chrono::NaiveDateTime;

#[post("/networks/actions")]
pub async fn post_networks_action(
    data: web::Data<AppData>,
    query: web::Json<NetworksAction>,
) -> HttpResponse {
    let params_map = QueryParamsAsMap::from(query.params.clone());

    let dns_server_result = datastore::load_all_dnsservers(&data.app_data_persistence).await;

    match query.action_type {
        NetworkActionType::AutoDiscover => {
            let network = params_map.get("network").unwrap();
            let lookup_names: bool = params_map.get("lookup_names").unwrap().parse().unwrap();

            if lookup_names && dns_server_result.is_err() {
                HttpResponse::InternalServerError().finish()
            } else {
                let mut dns_servers_found = false;
                let mut dns_servers_query_had_error = false;

                let dns_servers = match dns_server_result {
                    Ok(res) => {
                        dns_servers_found = !res.is_empty();
                        res
                    }
                    Err(err) => {
                        log::error!("Error during DNS server query: {}", err);
                        dns_servers_query_had_error = true;
                        vec![]
                    }
                };

                if dns_servers_query_had_error {
                    HttpResponse::InternalServerError().finish()
                } else if !dns_servers_found && lookup_names {
                    HttpResponse::BadRequest().body("No DNS Servers configured. Please configure DNS Servers first before doing a auto discovery with DNS lookup")
                } else {
                    match plugin_execution::auto_discover_servers_in_network(
                        network,
                        lookup_names,
                        dns_servers,
                    )
                    .await
                    {
                        Ok(list) => HttpResponse::Ok().json(list),
                        Err(_err) => HttpResponse::InternalServerError().finish(),
                    }
                }
            }
        }
    }
}

#[get("/servers")]
pub async fn get_servers(data: web::Data<AppData>) -> HttpResponse {
    match datastore::load_all_servers(&data.app_data_persistence, true).await {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(err) => HttpResponse::InternalServerError()
            .body(format!("Unexpected error occurred: {:?}", err)),
    }
}

#[post("/servers")]
pub async fn post_servers(data: web::Data<AppData>, query: web::Json<Server>) -> HttpResponse {
    match datastore::insert_server(&data.app_data_persistence, &query.0).await {
        Ok(result) => match result {
            true => HttpResponse::Ok().finish(),
            _ => HttpResponse::InternalServerError().body("Database could not be updated"),
        },
        Err(err) => HttpResponse::InternalServerError()
            .body(format!("Unexpected error occurred: {:?}", err)),
    }
}

#[post("/servers/actions")]
pub async fn post_servers_actions(
    data: web::Data<AppData>,
    query: web::Json<ServersAction>,
) -> HttpResponse {
    let params_map = QueryParamsAsMap::from(query.params.clone());

    match query.action_type {
        ServersActionType::Status => {
            let ips_to_check: Vec<IpAddr> = match params_map.get("ipaddresses") {
                Some(_list) => params_map
                    .get_split_by("ipaddresses", ",")
                    .unwrap()
                    .iter()
                    .flat_map(|s| {
                        let ip: Result<IpAddr, _> = s.parse();
                        ip
                    })
                    .collect(),
                None => Vec::new(),
            };

            let list = other_functions::statuscheck::status_check(ips_to_check, true)
                .await
                .unwrap();
            HttpResponse::Ok().json(list)
        }
        ServersActionType::FeatureScan => {
            match datastore::load_all_servers(&data.app_data_persistence, true).await {
                Ok(servers) => {
                    let upnp_activated =
                        !datastore::is_plugin_disabled("upnp", &data.app_data_persistence)
                            .await
                            .unwrap_or(true);

                    let list =
                        plugin_execution::discover_features_of_all_servers(servers, upnp_activated)
                            .await
                            .unwrap();

                    HttpResponse::Ok().json(list)
                }
                Err(err) => {
                    log::error!("Error while loading servers from database: {:?}", err);
                    HttpResponse::InternalServerError().body("Could not load servers from database")
                }
            }
        }
        ServersActionType::ActionConditionCheck => {
            HttpResponse::Ok().json(datastore::get_all_condition_results().to_vec())
        }
    }
}

#[post("/servers/{ipaddress}/actions")]
pub async fn post_servers_by_ipaddress_action(
    data: web::Data<AppData>,
    query: web::Json<ServerAction>,
    path: web::Path<String>,
) -> HttpResponse {
    let Ok(ipaddress): Result<IpAddr,_> = path.into_inner().parse() else {
        return HttpResponse::InternalServerError().body("IP Address is incorrect");
    };

    let server_res = datastore::get_server(&data.app_data_persistence, &ipaddress).await;

    if server_res.is_err() {
        return HttpResponse::InternalServerError()
            .body(format!("Server with ip {} not found", &ipaddress));
    }
    let server = server_res.unwrap();

    match query.action_type {
        ServerActionType::FeatureScan => {
            match plugin_execution::discover_features(ipaddress).await {
                Ok(list) => HttpResponse::Ok().json(list),
                Err(err) => HttpResponse::InternalServerError()
                    .body(format!("Unexpected error occurred: {:?}", err)),
            }
        }
        ServerActionType::Status => {
            match other_functions::statuscheck::status_check(vec![ipaddress], false).await {
                Ok(list) => HttpResponse::Ok().json(list.first().unwrap_or(&Status {
                    ipaddress,
                    is_running: false,
                })),
                Err(err) => HttpResponse::InternalServerError()
                    .body(format!("Unexpected error occurred: {:?}", err)),
            }
        }
        ServerActionType::ExecuteFeatureAction => {
            let params_map = QueryParamsAsMap::from(query.params.clone());

            let feature_id = params_map.get("feature_id").unwrap();
            let action_id: &String = params_map.get("action_id").unwrap();
            let action_params = params_map.get_as_str("action_params");

            let feature_res = server.find_feature(feature_id);

            if feature_res.is_none() {
                log::error!(
                    "Feature id {} not found for server {:?}",
                    feature_id,
                    server
                );
                return HttpResponse::InternalServerError()
                    .body(format!("Feature with id {} not known", feature_id));
            }
            let crypto_key = datastore::get_crypto_key();

            match plugin_execution::execute_action(
                &server,
                &feature_res.unwrap(),
                action_id,
                action_params,
                crypto_key,
            )
            .await
            {
                Ok(result) => HttpResponse::Ok().json(result),
                Err(err) => HttpResponse::InternalServerError()
                    .body(format!("Unexpected error occurred: {:?}", err)),
            }
        }
        ServerActionType::QueryData => {
            let crypto_key = datastore::get_crypto_key();

            match plugin_execution::execute_data_query(
                &server,
                &data.app_data_template_engine,
                crypto_key,
            )
            .await
            {
                Ok(results) => HttpResponse::Ok().json(results),
                Err(err) => HttpResponse::InternalServerError()
                    .body(format!("Unexpected error occurred: {:?}", err)),
            }
        }
    }
}

#[put("/servers/{ipaddress}")]
pub async fn put_servers_by_ipaddress(
    data: web::Data<AppData>,
    query: web::Json<Server>,
) -> HttpResponse {
    match datastore::update_server(&data.app_data_persistence, &query.0).await {
        Ok(result) => match result {
            true => HttpResponse::Ok().finish(),
            false => HttpResponse::InternalServerError().body("Database could not be updated"),
        },
        Err(err) => HttpResponse::InternalServerError()
            .body(format!("Unexpected error occurred: {:?}", err)),
    }
}

#[delete("/servers/{ipaddress}")]
pub async fn delete_servers_by_ipaddress(
    data: web::Data<AppData>,
    path: web::Path<String>,
) -> HttpResponse {
    let Ok(ipaddress) = path.into_inner().parse() else {
        return HttpResponse::InternalServerError().body("IP Address is incorrect");
    };

    match datastore::delete_server(&data.app_data_persistence, &ipaddress).await {
        Ok(result) => match result {
            true => HttpResponse::Ok().finish(),
            false => HttpResponse::InternalServerError().body("Database could not be updated"),
        },
        Err(err) => HttpResponse::InternalServerError()
            .body(format!("Unexpected error occurred: {:?}", err)),
    }
}

#[get("/plugins")]
pub async fn get_plugins(_data: web::Data<AppData>, _req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok().json(datastore::get_all_plugins())
}

#[get("/plugins/actions")]
pub async fn get_plugins_actions(
    data: web::Data<AppData>,
    query: web::Query<std::collections::HashMap<String, String>>,
) -> HttpResponse {
    let params = query.into_inner();

    match params.get("query") {
        Some(query_value) => match query_value.as_str() {
            "disabled" => {
                let persistence = &data.app_data_persistence;

                match datastore::get_disabled_plugins(persistence).await {
                    Ok(list) => HttpResponse::Ok().json(list),
                    Err(_err) => {
                        HttpResponse::InternalServerError().body("Cannot load disabled plugins")
                    }
                }
            }
            y => {
                log::error!("Query param value {} not known", y);
                HttpResponse::BadRequest().finish()
            }
        },
        None => {
            log::error!("No query parameter found. Request is invalid");
            HttpResponse::BadRequest().finish()
        }
    }
}

#[put("/plugins/actions")]
pub async fn put_plugins_actions(
    data: web::Data<AppData>,
    query: web::Json<PluginsAction>,
) -> HttpResponse {
    let action = query.into_inner();
    let params_map = QueryParamsAsMap::from(action.params);

    let persistence = &data.app_data_persistence;

    match datastore::disable_plugins(persistence, params_map.get_split_by("ids", ",").unwrap())
        .await
    {
        Ok(res) => match res {
            true => HttpResponse::Ok().finish(),
            false => HttpResponse::InternalServerError().body("Cannot disable plugins"),
        },
        Err(err) => {
            log::error!("Error {:?}", err);
            HttpResponse::InternalServerError()
                .body("Unknown error while trying to disable plugins")
        }
    }
}

#[post("/configurations/dnsservers")]
pub async fn post_dnsservers(
    data: web::Data<AppData>,
    query: web::Json<DNSServer>,
) -> HttpResponse {
    let persistence = &data.app_data_persistence;

    let server = query.into_inner();

    match datastore::insert_dnsserver(persistence, &server).await {
        Ok(_res) => HttpResponse::Ok().finish(),
        Err(_err) => HttpResponse::InternalServerError().body("Cannot save DNS server"),
    }
}

#[get("/systeminformation/dnsservers")]
pub async fn get_system_dnsservers(_data: web::Data<AppData>) -> HttpResponse {
    HttpResponse::Ok().json(systeminfo::get_systenms_dns_servers())
}

#[get("/configurations/dnsservers")]
pub async fn get_dnsservers(data: web::Data<AppData>) -> HttpResponse {
    let persistence = &data.app_data_persistence;

    match datastore::load_all_dnsservers(persistence).await {
        Ok(list) => HttpResponse::Ok().json(list),
        Err(_err) => HttpResponse::InternalServerError().body("Cannot load DNS servers"),
    }
}

#[delete("/configurations/dnsservers/{ipaddress}")]
pub async fn delete_dnsservers(data: web::Data<AppData>, path: web::Path<String>) -> HttpResponse {
    let persistence = &data.app_data_persistence;

    let Ok(ipaddress): Result<IpAddr,_> = path.into_inner().parse() else {
        return HttpResponse::InternalServerError().body("IP Address is incorrect");
    };

    match datastore::delete_dnsserver(persistence, &ipaddress).await {
        Ok(_res) => HttpResponse::Ok().finish(),
        Err(_err) => HttpResponse::InternalServerError().body("Cannot save DNS server"),
    }
}

#[get("/system/information")]
pub async fn get_system_information() -> HttpResponse {
    HttpResponse::Ok().json(SystemInformation {
        load_average: systeminfo::get_load_info(),
        memory_stats: systeminfo::get_memory_stats(),
        memory_usage: systeminfo::get_memory_usage(),
    })
}

#[get("/system/smtpconfigvalid")]
pub async fn get_smtp_config_valid() -> HttpResponse {
    HttpResponse::Ok().json(common::is_smtp_config_valid())
}

#[get("/users")]
pub async fn get_users(data: web::Data<AppData>) -> HttpResponse {
    match datastore::load_all_users(&data.app_data_persistence).await {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(err) => HttpResponse::InternalServerError()
            .body(format!("Unexpected error occurred: {:?}", err)),
    }
}

#[get("/users/exist")]
pub async fn get_users_exist(data: web::Data<AppData>) -> HttpResponse {
    match datastore::load_all_users(&data.app_data_persistence).await {
        Ok(result) => HttpResponse::Ok().json(!result.is_empty()),
        Err(err) => HttpResponse::InternalServerError()
            .body(format!("Unexpected error occurred: {:?}", err)),
    }
}

#[post("/users")]
pub async fn post_user(data: web::Data<AppData>, query: web::Json<User>) -> HttpResponse {
    save_user_common(data, query).await
}

#[post("/users_first")]
pub async fn post_first_user(data: web::Data<AppData>, query: web::Json<User>) -> HttpResponse {
    match datastore::load_all_users(&data.app_data_persistence).await {
        Ok(result) => {
            if !result.is_empty() {
                log::error!("Called function that is used for initial user save that allows and update without authorization. However, there are already users. So this is not the initial user creation.");
                HttpResponse::Unauthorized().finish()
            } else {
                save_user_common(data, query).await
            }
        }
        Err(err) => HttpResponse::InternalServerError()
            .body(format!("Unexpected error occurred: {:?}", err)),
    }
}

async fn save_user_common(data: web::Data<AppData>, query: web::Json<User>) -> HttpResponse {
    let initial_password = common::generate_short_random_string();

    let Ok(password_hash) = common::hash_password(initial_password.as_str()) else {
        log::error!("Could not generate password hash");
        return HttpResponse::InternalServerError().finish();
    };

    let mut user = query.0;
    user.update_password_hash(password_hash);

    let Ok(update_result) = datastore::insert_user(&data.app_data_persistence, &user).await else {
        log::error!("Could not save user {:?}", &user);
        return HttpResponse::InternalServerError().finish();
    };

    if update_result {
        if common::is_smtp_config_valid() {
            let from_address = datastore::get_config().get_string("email_from").unwrap();

            match common::send_email(from_address.as_str(),
                    &user.get_email(),
                    "Your initial password for the Server-Manager",
                    format!("Hello {},\n\nyour user id is '{}' and the initial password is: '{}'.\n\nRegards,\nyour Server-Manager", user.get_full_name(), user.get_user_id(), initial_password).as_str()).await {
                        Ok(res) => {
                            if res {
                                HttpResponse::Ok().finish() // mail send successfully - no need to display the initual password
                            }
                            else {
                                HttpResponse::Ok().json(initial_password)
                            }
                        },
                        Err(err) => {
                            log::error!("An error occurred while trying to send the initial password mail: {}", err);
                            HttpResponse::Ok().json(initial_password)
                        }
                    }
        } else {
            HttpResponse::Ok().json(initial_password)
        }
    } else {
        HttpResponse::InternalServerError().finish()
    }
}

#[delete("/users/{user_id}")]
pub async fn delete_user(data: web::Data<AppData>, path: web::Path<String>) -> HttpResponse {
    let user_id = path.into_inner();

    match datastore::delete_user(&data.app_data_persistence, &user_id).await {
        Ok(result) => match result {
            true => HttpResponse::Ok().finish(),
            false => HttpResponse::InternalServerError().body("Database could not be updated"),
        },
        Err(err) => HttpResponse::InternalServerError()
            .body(format!("Unexpected error occurred: {:?}", err)),
    }
}

#[put("/user/{user_id}/changepassword")]
pub async fn put_user_changepassword(
    data: web::Data<AppData>,
    req: HttpRequest,
    query: web::Json<PasswordChange>,
) -> HttpResponse {
    let headers = req.headers();
    let custom_header = headers.get(HeaderName::from_static("x-custom"));

    let Some(custom_header) = custom_header else {
            log::error!("Could not get Custom HeaderValue X-custom ");
            return HttpResponse::Unauthorized().finish();
        };

    let Ok(otk_tuple) = get_existing_otk(custom_header) else {
            log::error!("Could not get OneTimeToken with id {:?}", custom_header);
            return HttpResponse::Unauthorized().finish();
        };

    let secret = common::make_aes_secrect(query.user_id.as_str(), otk_tuple.1.as_str());

    let Ok(decrypted_old) = common::aes_decrypt(&query.old_password, secret.as_str()) else {
            log::error!("Could not decrypt old password");
            return HttpResponse::Unauthorized().finish();
        };

    let Ok(mut user) = datastore::get_user(&data.app_data_persistence, query.user_id.as_str()).await else {
            log::error!("Could not find user {}", query.user_id);
            return HttpResponse::Unauthorized().finish();
        };

    let Ok(password_check_result) = user.check_password(&decrypted_old) else {
            log::error!("Could not check old password of user {}", query.user_id);
            return HttpResponse::Unauthorized().finish();
        };

    if password_check_result {
        let Ok(decrypted_new) = common::aes_decrypt(&query.new_password, secret.as_str()) else {
                log::error!("Could not decrypt new password");
                return HttpResponse::Unauthorized().finish();
            };

        user.update_password_hash(common::hash_password(decrypted_new.as_str()).unwrap());

        let Ok(user_updated) = datastore::update_user(&data.app_data_persistence, &user).await else {
                log::error!("User was not updated");
                return HttpResponse::Unauthorized().finish();
            };

        if user_updated {
            HttpResponse::Ok().finish();
        }
    } else {
        log::error!("Password check of old password failed");
    }

    HttpResponse::Unauthorized().finish()
}

#[get("users/authenticate/otk")]
pub async fn get_one_time_key() -> HttpResponse {
    let otk = OneTimeKey::generate();

    HttpResponse::Ok().json(otk)
}

#[post("users/authenticate")]
pub async fn authenticate(data: web::Data<AppData>, req: HttpRequest) -> HttpResponse {
    let headers = req.headers();
    let auth_header = headers.get(header::AUTHORIZATION);
    let custom_header = headers.get(HeaderName::from_static("x-custom"));

    if custom_header.is_some() && auth_header.is_some() {
        let Ok(otk_tuple) = get_existing_otk(custom_header.unwrap()) else {
            log::error!("Could not get OneTimeToken with id {:?}", custom_header);
            return HttpResponse::Unauthorized().finish();
        };

        let Ok(auth_values_opt) = get_auth_data_split(auth_header.unwrap()) else {
            log::error!("Could not get authorization header");
            return HttpResponse::Unauthorized().finish();
        };

        let Some(auth_values) = auth_values_opt else {
            log::error!("Could not get authorization header");
            return HttpResponse::Unauthorized().finish();
        };

        let user_id = auth_values.0;
        let secret = common::make_aes_secrect(user_id.as_str(), otk_tuple.1.as_str());

        let Ok(decrypted) = common::aes_decrypt(&auth_values.1, secret.as_str()) else {
            log::error!("Could not decrypt password");
            return HttpResponse::Unauthorized().finish();
        };

        let Ok( user) = datastore::get_user(&data.app_data_persistence, user_id.as_str()).await else {
            log::error!("Could not find user {}",user_id );
            return HttpResponse::Unauthorized().finish();
        };

        let Ok(password_check_result) = user.check_password(&decrypted) else {
            log::error!("Could not check password of user {}", user_id);
            return HttpResponse::Unauthorized().finish();
        };

        if password_check_result {
            let token = common::generate_long_random_string();

            datastore::insert_token(&token);

            return HttpResponse::Ok().json(UserToken { user_id, token });
        } else {
            return HttpResponse::Unauthorized().body("The given password was invalid");
        }
    }
    HttpResponse::Unauthorized().finish()
}

fn get_existing_otk(header_value: &HeaderValue) -> Result<(NaiveDateTime, String), AppError> {
    let number: u32 = header_value.to_str()?.parse()?;

    common::OneTimeKey::get_token(number).ok_or(AppError::UnAuthorized)
}

fn get_auth_data_split(header_value: &HeaderValue) -> Result<Option<(String, String)>, AppError> {
    let val_str = header_value.to_str().unwrap();
    let cut_val_str = val_str.replace("Basic ", "");

    let decoded = common::decode_base64_urlsafe_with_pad(cut_val_str.as_str());

    Ok(decoded
        .split_once(':')
        .map(|v| (v.0.to_owned(), v.1.to_owned())))
}
