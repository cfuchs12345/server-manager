use actix_web::HttpResponse;
use actix_web::{middleware::Logger, web, App, HttpServer, HttpRequest, Result};
use actix_files as fs;
use handlebars::no_escape;
use core::panic;
use std::path::PathBuf;
use config::Config;

use crate::appdata::AppData;
use crate::handlebars_helpers;
use crate::routes;
use crate::persistence;


#[actix_web::main]
pub async fn start() -> std::io::Result<()> {
    let config = get_config();
    let bind_address = config.get_string("bind_address").unwrap();
    let db_url = config.get_string("db_url").unwrap();
    let template_base_path = config.get_string("template_base_path").unwrap();


    log::debug!("dir: {}", template_base_path);


    let persistence = persistence::Persistence::new(&db_url).await;
    
    let template_engine = create_and_configure_template_engine(&template_base_path);

    let app_data = AppData { app_data_config: config, app_data_persistence: persistence, app_data_template_engine: template_engine };

    HttpServer::new(move || {
        App::new()
            
            .wrap(Logger::default())
            .app_data(web::Data::new(app_data.clone()))
            .configure(init)
    })
    .bind(bind_address)?
    .run()
    .await
}

fn create_and_configure_template_engine(template_base_path: &str) -> handlebars::Handlebars<'static> {
    let mut handlebars = handlebars::Handlebars::new();
    handlebars
        .register_templates_directory(".html", template_base_path)
        .unwrap();    
    handlebars.set_dev_mode(true);
    

    let templates = handlebars.get_templates();
    for entry in  templates {
        log::debug!("registered template: {}", entry.0);
    }

    handlebars.set_strict_mode(true);
    handlebars.register_escape_fn(no_escape); //html escaping is the default and cause issue
    handlebars_helpers::register(&mut handlebars);

    handlebars
}

async fn fav_icon(_req: HttpRequest) -> Result<fs::NamedFile> {
    handle_named_file("./server/static/images/favicon.ico")
}

async fn index_html(_req: HttpRequest) -> Result<fs::NamedFile> {
    handle_named_file("./server/static/index.html")
}

async fn named_file(req: HttpRequest) -> Result<fs::NamedFile> {
    let path: PathBuf = req.match_info().query("filename").parse().unwrap();
    handle_named_file(format!("./server/static/{}", path.as_os_str().to_str().unwrap()).as_str())
}

async fn named_file_svg(req: HttpRequest) -> Result<fs::NamedFile> {
    let path: PathBuf = req.match_info().query("filename").parse().unwrap();
    handle_named_file(format!("./server/static/assets/svg/{}", path.as_os_str().to_str().unwrap()).as_str())
}



fn handle_named_file(file: &str) -> std::result::Result<fs::NamedFile, actix_web::Error> {
    let path_found = fs::NamedFile::open(file);
    match path_found {
        Ok(f) => Ok(f),
        Err(e) => Err(actix_web::error::ErrorNotFound(""))
    }
}


fn init(cfg: &mut web::ServiceConfig) {
    // Files for frontend ==>
    cfg.route("/favicon.ico", web::get().to(fav_icon));
    cfg.route("/", web::get().to(index_html));
    cfg.route("/index.html", web::get().to(index_html));
    cfg.route("/{filename:main.*\\.js}", web::get().to(named_file));
    cfg.route("/{filename:polyfills.*\\.js}", web::get().to(named_file));
    cfg.route("/{filename:runtime.*\\.js}", web::get().to(named_file));
    cfg.route("/{filename:styles.*}", web::get().to(named_file));
    cfg.route("/{filename:3rdpartylicenses.txt}", web::get().to(named_file));

    cfg.route("/assets/svg/{filename:.*\\.svg}", web::get().to(named_file_svg));
    // <== files for frontend
    
    
    cfg.service(routes::post_networks_action);
    
    cfg.service(routes::post_servers);
    cfg.service(routes::get_servers);
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
}


pub fn get_config() -> Config {
    Config::builder()
            .add_source(config::Environment::default())
            .build()
            .expect("Could not load config from env properties") // ok to panic, if the config cannot be loaded
}