use actix_web::delete;
use actix_web::{web, get, put, post, HttpRequest, HttpResponse};

use crate::appdata::AppData;
use crate::config_types::DNSServer;
use crate::{plugins, features, config, status, types, inmemory};
use crate::discover::{self};
use crate::servers;
use crate::types::{Status, NetworkActionType, ServersActionType, ServersAction, ServerAction, ServerActionType, PluginsAction, NetworksAction};
use crate::server_types::{Server};


#[post("/backend/networks/actions")]
pub async fn post_networks_action(data: web::Data<AppData>, query: web::Json<NetworksAction>)  ->  HttpResponse {
    let params_map = types::QueryParamsAsMap::from(query.params.clone());

    let dns_server_result = config::load_all_dnsservers(&data.app_data_persistence).await;

    
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
                    match discover::auto_discover_servers_in_network(network, lookup_names, dns_servers).await {
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
    match servers::load_all_servers(&data.app_data_persistence, true).await {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(err) => HttpResponse::InternalServerError().body(format!("Unexpected error occurred: {:?}", err))
    }    
}

#[post("/backend/servers")]
pub async fn post_servers(data: web::Data<AppData>, query: web::Json<Server>) -> HttpResponse {
    match servers::save_server(&data.app_data_persistence, &query.0).await {
        Ok(result) => match result  {
            true => HttpResponse::Ok().finish(),
             _ => HttpResponse::InternalServerError().body("Database could not be updated")
        },
        Err(err) => HttpResponse::InternalServerError().body(format!("Unexpected error occurred: {:?}", err))
    }
}

#[post("/backend/servers/actions")]
pub async fn post_servers_actions(data: web::Data<AppData>, query: web::Json<ServersAction>) -> HttpResponse {
    let params_map = types::QueryParamsAsMap::from(query.params.clone());

    match query.action_type {
        ServersActionType::Status => {
            let ips_to_check = match params_map.get("ip_addresses") {
                Some(_list) => params_map.get_split_by("ip_addresses", ",").unwrap(),
                None => Vec::new()
            };
            
            let list = status::status_check(ips_to_check, true).await.unwrap();
            HttpResponse::Ok().json(list)
        },
        ServersActionType::FeatureScan => {
            match servers::load_all_servers(&data.app_data_persistence, true).await {                
                Ok(servers) => {
                    let upnp_activated = !plugins::is_plugin_disabled("upnp", &data.app_data_persistence).await.unwrap_or(true);
                    let list = discover::discover_features_of_all_servers(servers, upnp_activated).await.unwrap();
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
            HttpResponse::Ok().json(inmemory::get_all_condition_results().to_vec())
        }
    }   
}

#[post("/backend/servers/{ipaddress}/actions")]
pub async fn post_servers_by_ipaddress_action(data: web::Data<AppData>, query: web::Json<ServerAction>, path: web::Path<String>) -> HttpResponse {
    let ipaddress = path.into_inner();

    let server_res = servers::get_server(&data.app_data_persistence, ipaddress.clone()).await;
    
    if server_res.is_err() {
        return HttpResponse::InternalServerError().body(format!("Server with ip {} not found", &ipaddress));
    }
    let server = server_res.unwrap();


    match query.action_type {
        ServerActionType::FeatureScan => {
            match discover::discover_features(&ipaddress).await {
                Ok(list) => HttpResponse::Ok().json(list),
                Err(err) =>  HttpResponse::InternalServerError().body(format!("Unexpected error occurred: {:?}", err))
            }            
        },
        ServerActionType::Status => {
            match status::status_check(vec![ipaddress.clone()], false).await {
                Ok(list) => HttpResponse::Ok().json(list.first().unwrap_or(&Status {
                    ipaddress,
                    is_running: false
                })),
                Err(err) =>  HttpResponse::InternalServerError().body(format!("Unexpected error occurred: {:?}", err))
            }
        },
        ServerActionType::ExecuteFeatureAction => {
            let params_map = types::QueryParamsAsMap::from(query.params.clone());
            
            let feature_id = params_map.get("feature_id").unwrap();
            let action_id: &String = params_map.get("action_id").unwrap();

            let feature_res = server.find_feature(feature_id.clone());

            if feature_res.is_none() {
                return HttpResponse::InternalServerError().body(format!("Feature with id {} not known", feature_id));
            }

            let crypto_key = inmemory::get_crypto_key();

            match features::execute_action(&server, feature_res.unwrap(), action_id, crypto_key).await {
                Ok(result) => HttpResponse::Ok().json(result),
                Err(err) =>  HttpResponse::InternalServerError().body(format!("Unexpected error occurred: {:?}", err))
            }
        },
        ServerActionType::ActionConditionCheck => {
            let params_map = types::QueryParamsAsMap::from(query.params.clone());
            
            let feature_id = params_map.get("feature_id").unwrap();
            let action_id: &String = params_map.get("action_id").unwrap();

            let feature_res = server.find_feature(feature_id.clone());

            if feature_res.is_none() {
                return HttpResponse::InternalServerError().body(format!("Feature with id {} not known", feature_id));
            }

            let plugin_opt = inmemory::get_plugin(feature_res.unwrap().id.as_str());

            if plugin_opt.is_none() {
                return HttpResponse::InternalServerError().body(format!("Plugin with id {} not known", feature_id));
            }
            let plugin = plugin_opt.unwrap();

            let action_opt = plugin.actions.iter().find(|a| a.id == *action_id);

            if action_opt.is_none() {
                return HttpResponse::InternalServerError().body(format!("Action with id {} not known for plugin with id {}", feature_id, plugin.id));
            }

            let crypto_key = inmemory::get_crypto_key();


            let result = features::check_condition_for_action_met( &server, feature_res.unwrap(), action_opt.unwrap(), crypto_key).await;
            HttpResponse::Ok().json(result.result)
        }
        ServerActionType::QueryData => {       
            let crypto_key = inmemory::get_crypto_key();

            match features::execute_data_query(&server, &data.app_data_template_engine, crypto_key).await {
                Ok(results) => {
                    HttpResponse::Ok().json(results)
                }
                Err(err) =>  HttpResponse::InternalServerError().body(format!("Unexpected error occurred: {:?}", err))
            }
        },
        ServerActionType::QueryDependencyData => {
            todo!("needed?")
        },
        ServerActionType::IsConditionForFeatureActionMet => {
            let params_map = types::QueryParamsAsMap::from(query.params.clone());
            
            let feature_id = params_map.get("feature_id").unwrap();
            let action_id: &String = params_map.get("action_id").unwrap();
            

            let feature_res = server.find_feature(feature_id.clone());
           
            if feature_res.is_none() {
                return HttpResponse::InternalServerError().body(format!("Feature with id {} not known", feature_id));
            }
            
            let plugin_opt = inmemory::get_plugin(feature_res.unwrap().id.as_str());

            if plugin_opt.is_none() {
                return HttpResponse::InternalServerError().body(format!("Plugin with id {} not known", feature_id));
            }

            let plugin = plugin_opt.unwrap();

            let action_opt = plugin.actions.iter().find(|a| a.id == *action_id);

            if action_opt.is_none() {
                return HttpResponse::InternalServerError().body(format!("Action with id {} not known for plugin with id {}", feature_id, plugin.id));
            }
            let crypto_key = inmemory::get_crypto_key();

            let result = features::check_condition_for_action_met(&server, feature_res.unwrap(), action_opt.unwrap(), crypto_key).await;
            HttpResponse::Ok().json(result.result)

        }
    }  
}



#[put("/backend/servers/{ipaddress}")]
pub async fn put_servers_by_ipaddress(data: web::Data<AppData>, query: web::Json<Server>) -> HttpResponse {
    match servers::update_server(&data.app_data_persistence, &query.0).await {
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
    
    match servers::delete_server(&data.app_data_persistence, &ipaddress).await {
        Ok(result) => match result {
            true => HttpResponse::Ok().finish(),
            false =>  HttpResponse::InternalServerError().body("Database could not be updated")
        },
        Err(err) =>  HttpResponse::InternalServerError().body(format!("Unexpected error occurred: {:?}", err))
    }
}



#[get("/backend/plugins")]
pub async fn get_plugins(_data: web::Data<AppData>, _req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok().json(plugins::get_all_plugins())
}

#[get("/backend/plugins/actions")]
pub async fn get_plugins_actions(data: web::Data<AppData>, query: web::Query<std::collections::HashMap<String, String>>) -> HttpResponse {
    let params = query.into_inner();

    match params.get("query") {
        Some(query_value) => {
            match query_value.as_str() {
                "disabled" => {
                    let persistence = &data.app_data_persistence;

                    match plugins::get_disabled_plugins(persistence).await {
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
    let params_map = types::QueryParamsAsMap::from(action.params);

    let persistence = &data.app_data_persistence;

    match plugins::disable_plugins(persistence, params_map.get_split_by("ids", ",").unwrap()).await {
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

    match config::save_dnsserver(persistence, &server).await {
        Ok(_res) => HttpResponse::Ok().finish(),
        Err(_err) => HttpResponse::InternalServerError().body("Cannot save DNS server")
    }
}

#[get("/backend/configurations/dnsservers")]
pub async fn get_dnsservers(data: web::Data<AppData>)  -> HttpResponse {
    let persistence = &data.app_data_persistence;

    match config::load_all_dnsservers(persistence).await {
        Ok(list) => HttpResponse::Ok().json(list),
        Err(_err) => HttpResponse::InternalServerError().body("Cannot load DNS servers")
    }
}

#[delete("/backend/configurations/dnsservers/{ipaddress}")]
pub async fn delete_dnsservers(data: web::Data<AppData>, path: web::Path<String>)  -> HttpResponse {
    let persistence = &data.app_data_persistence;
    let ipaddress = path.into_inner();

    match config::delete_dnsserver(persistence, &ipaddress).await {
        Ok(_res) => HttpResponse::Ok().finish(),
        Err(_err) => HttpResponse::InternalServerError().body("Cannot save DNS server")
    }
}

