use actix_web::delete;
use actix_web::{web, get, put, post, HttpRequest, HttpResponse};
use crate::commands::ping;
use crate::models::config::dns_server::DNSServer;
use crate::models::request::plugin::PluginsAction;
use crate::models::request::server::{ServersAction, ServersActionType, ServerActionType, ServerAction, NetworksAction, NetworkActionType};
use crate::models::request::common::QueryParamsAsMap;
use crate::models::response::status::Status;
use crate::models::response::system_information::SystemInformation;
use crate::models::server::Server;
use crate::webserver::appdata::AppData;
use crate::{other_functions::systeminfo, datastore, plugin_execution};



#[post("/backend/networks/actions")]
pub async fn post_networks_action(data: web::Data<AppData>, query: web::Json<NetworksAction>)  ->  HttpResponse {
    let params_map = QueryParamsAsMap::from(query.params.clone());

    let dns_server_result = datastore::load_all_dnsservers(&data.app_data_persistence).await;

    
    match query.action_type {
        NetworkActionType::AutoDiscover =>  {
            let network = params_map.get("network").unwrap();
            let lookup_names: bool = params_map.get("lookup_names").unwrap().parse().unwrap();
            
            if lookup_names && dns_server_result.is_err() {
                HttpResponse::InternalServerError().finish()
            }          
            else {
                let mut dns_servers_found = false;
                let mut dns_servers_query_had_error = false;               

                let dns_servers = match dns_server_result {
                    Ok(res) => {
                        dns_servers_found = !res.is_empty();
                        res
                    },
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
                    match plugin_execution::auto_discover_servers_in_network(network, lookup_names, dns_servers).await {
                        Ok(list) =>  HttpResponse::Ok().json(list),
                        Err(_err) => HttpResponse::InternalServerError().finish()
                    }
                }

            }        
        }  
    } 
}

#[get("/backend/servers")]
pub async fn get_servers(data: web::Data<AppData<>>) -> HttpResponse {
//    let server_list = servers::get_all.await;
    match datastore::load_all_servers(&data.app_data_persistence, true).await {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(err) => HttpResponse::InternalServerError().body(format!("Unexpected error occurred: {:?}", err))
    }    
}

#[post("/backend/servers")]
pub async fn post_servers(data: web::Data<AppData>, query: web::Json<Server>) -> HttpResponse {
    match datastore::insert_server(&data.app_data_persistence, &query.0).await {
        Ok(result) => match result  {
            true => HttpResponse::Ok().finish(),
             _ => HttpResponse::InternalServerError().body("Database could not be updated")
        },
        Err(err) => HttpResponse::InternalServerError().body(format!("Unexpected error occurred: {:?}", err))
    }
}

#[post("/backend/servers/actions")]
pub async fn post_servers_actions(data: web::Data<AppData>, query: web::Json<ServersAction>) -> HttpResponse {
    let params_map = QueryParamsAsMap::from(query.params.clone());

    match query.action_type {
        ServersActionType::Status => {
            let ips_to_check = match params_map.get("ip_addresses") {
                Some(_list) => params_map.get_split_by("ip_addresses", ",").unwrap(),
                None => Vec::new()
            };
            
            let list = ping::status_check(ips_to_check, true).await.unwrap();
            HttpResponse::Ok().json(list)
        },
        ServersActionType::FeatureScan => {
            match datastore::load_all_servers(&data.app_data_persistence, true).await {                
                Ok(servers) => {
                    let upnp_activated = !datastore::is_plugin_disabled("upnp", &data.app_data_persistence).await.unwrap_or(true);
                    let list = plugin_execution::discover_features_of_all_servers(servers, upnp_activated).await.unwrap();
                    log::debug!("list of found features: {:?}", list);
                    HttpResponse::Ok().json(list)       
                 
                },
                Err(err) => {
                    log::error!("Error while loading servers from database: {:?}", err);
                    HttpResponse::InternalServerError().body("Could not load servers from database")
                }
                        
            }
        },
        ServersActionType::ActionConditionCheck => {
            HttpResponse::Ok().json(datastore::get_all_condition_results().to_vec())
        }
    }   
}

#[post("/backend/servers/{ipaddress}/actions")]
pub async fn post_servers_by_ipaddress_action(data: web::Data<AppData>, query: web::Json<ServerAction>, path: web::Path<String>) -> HttpResponse {
    let ipaddress = path.into_inner();

    let server_res = datastore::get_server(&data.app_data_persistence, ipaddress.clone()).await;
    
    if server_res.is_err() {
        return HttpResponse::InternalServerError().body(format!("Server with ip {} not found", &ipaddress));
    }
    let server = server_res.unwrap();


    match query.action_type {
        ServerActionType::FeatureScan => {
            match plugin_execution::discover_features(&ipaddress).await {
                Ok(list) => HttpResponse::Ok().json(list),
                Err(err) =>  HttpResponse::InternalServerError().body(format!("Unexpected error occurred: {:?}", err))
            }            
        },
        ServerActionType::Status => {
            match ping::status_check(vec![ipaddress.clone()], false).await {
                Ok(list) => HttpResponse::Ok().json(list.first().unwrap_or(&Status {
                    ipaddress,
                    is_running: false
                })),
                Err(err) =>  HttpResponse::InternalServerError().body(format!("Unexpected error occurred: {:?}", err))
            }
        },
        ServerActionType::ExecuteFeatureAction => {
            let params_map = QueryParamsAsMap::from(query.params.clone());
            
            let feature_id = params_map.get("feature_id").unwrap();
            let action_id: &String = params_map.get("action_id").unwrap();
            let action_params = params_map.get_as_str("action_params");
            
            let feature_res = server.find_feature(feature_id);

            if feature_res.is_none() {
                log::error!("Feature id {} not found for server {:?}", feature_id, server);
                return HttpResponse::InternalServerError().body(format!("Feature with id {} not known", feature_id));
            }
            let crypto_key = datastore::get_crypto_key();

            match plugin_execution::execute_action(&server, &feature_res.unwrap(), action_id, action_params, crypto_key).await {
                Ok(result) => HttpResponse::Ok().json(result),
                Err(err) =>  HttpResponse::InternalServerError().body(format!("Unexpected error occurred: {:?}", err))
            }
        },        
        ServerActionType::QueryData => {       
            let crypto_key = datastore::get_crypto_key();

            match plugin_execution::execute_data_query(&server, &data.app_data_template_engine, crypto_key).await {
                Ok(results) => {
                    log::info!("{:?}", results);
                    HttpResponse::Ok().json(results)
                }
                Err(err) =>  HttpResponse::InternalServerError().body(format!("Unexpected error occurred: {:?}", err))
            }
        }        
    }  
}



#[put("/backend/servers/{ipaddress}")]
pub async fn put_servers_by_ipaddress(data: web::Data<AppData>, query: web::Json<Server>) -> HttpResponse {
    match datastore::update_server(&data.app_data_persistence, &query.0).await {
        Ok(result) => match result {
            true => HttpResponse::Ok().finish(),
            false =>  HttpResponse::InternalServerError().body("Database could not be updated")
        },
        Err(err) =>  HttpResponse::InternalServerError().body(format!("Unexpected error occurred: {:?}", err))
    }
}

#[delete("/backend/servers/{ipaddress}")]
pub async fn delete_servers_by_ipaddress(data: web::Data<AppData>,  path: web::Path<String>) -> HttpResponse {
    let ipaddress = path.into_inner();
    
    match datastore::delete_server(&data.app_data_persistence, &ipaddress).await {
        Ok(result) => match result {
            true => HttpResponse::Ok().finish(),
            false =>  HttpResponse::InternalServerError().body("Database could not be updated")
        },
        Err(err) =>  HttpResponse::InternalServerError().body(format!("Unexpected error occurred: {:?}", err))
    }
}



#[get("/backend/plugins")]
pub async fn get_plugins(_data: web::Data<AppData>, _req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok().json(datastore::get_all_plugins())
}

#[get("/backend/plugins/actions")]
pub async fn get_plugins_actions(data: web::Data<AppData>, query: web::Query<std::collections::HashMap<String, String>>) -> HttpResponse {
    let params = query.into_inner();

    match params.get("query") {
        Some(query_value) => {
            match query_value.as_str() {
                "disabled" => {
                    let persistence = &data.app_data_persistence;

                    match datastore::get_disabled_plugins(persistence).await {
                        Ok(list) =>  HttpResponse::Ok().json(list),
                        Err(_err) => 
                            HttpResponse::InternalServerError().body("Cannot load disabled plugins")        
                    }
                },
                y => {
                    log::error!("Query param value {} not known", y);
                    HttpResponse::BadRequest().finish()
                }
            }
        },
        None => {
            log::error!("No query parameter found. Request is invalid");
            HttpResponse::BadRequest().finish()
        }
    }    
}

#[put("/backend/plugins/actions")]
pub async fn put_plugins_actions(data: web::Data<AppData>,  query: web::Json<PluginsAction>) -> HttpResponse {
    let action = query.into_inner();
    let params_map = QueryParamsAsMap::from(action.params);

    let persistence = &data.app_data_persistence;

    match datastore::disable_plugins(persistence, params_map.get_split_by("ids", ",").unwrap()).await {
        Ok(res) => {
            match res {
                true => HttpResponse::Ok().finish(),
                false => HttpResponse::InternalServerError().body("Cannot disable plugins")
            }            
        },
        Err(err) => {
            log::error!("Error {:?}", err);
            HttpResponse::InternalServerError().body("Unknown error while trying to disable plugins")
        }
    }
}

#[post("/backend/configurations/dnsservers")]
pub async fn post_dnsservers(data: web::Data<AppData>,  query: web::Json<DNSServer>)  -> HttpResponse {
    let persistence = &data.app_data_persistence;

    let server = query.into_inner();

    match datastore::insert_dnsserver(persistence, &server).await {
        Ok(_res) => HttpResponse::Ok().finish(),
        Err(_err) => HttpResponse::InternalServerError().body("Cannot save DNS server")
    }
}

#[get("/backend/systeminformation/dnsservers")]
pub async fn get_system_dnsservers(_data: web::Data<AppData>)  -> HttpResponse {
    HttpResponse::Ok().json( systeminfo::get_systenms_dns_servers() )
}


#[get("/backend/configurations/dnsservers")]
pub async fn get_dnsservers(data: web::Data<AppData>)  -> HttpResponse {
    let persistence = &data.app_data_persistence;

    match datastore::load_all_dnsservers(persistence).await {
        Ok(list) => HttpResponse::Ok().json(list),
        Err(_err) => HttpResponse::InternalServerError().body("Cannot load DNS servers")
    }
}

#[delete("/backend/configurations/dnsservers/{ipaddress}")]
pub async fn delete_dnsservers(data: web::Data<AppData>, path: web::Path<String>)  -> HttpResponse {
    let persistence = &data.app_data_persistence;
    let ipaddress = path.into_inner();

    match datastore::delete_dnsserver(persistence, &ipaddress).await {
        Ok(_res) => HttpResponse::Ok().finish(),
        Err(_err) => HttpResponse::InternalServerError().body("Cannot save DNS server")
    }
}

#[get("/backend/system/information")]
pub async fn get_system_information() -> HttpResponse {
      HttpResponse::Ok().json(SystemInformation {
        load_average: systeminfo::get_load_info(),
        memory_stats: systeminfo::get_memory_stats(),
        memory_usage: systeminfo::get_memory_usage()
    })
}