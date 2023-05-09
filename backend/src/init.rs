use actix_files as fs;
use actix_web::{middleware::Logger, web, App, HttpRequest, HttpServer, Result};
use config::Config;
use handlebars::no_escape;
use std::path::Path;
use std::path::PathBuf;
use tokio_cron_scheduler::{Job, JobScheduler};

use crate::appdata::AppData;
use crate::crypt;
use crate::features;
use crate::handlebars_helper_functions;
use crate::inmemory;
use crate::migrations;
use crate::persistence;
use crate::persistence::Entry;
use crate::persistence::Persistence;
use crate::plugins;
use crate::plugins::init_cache_silent;
use crate::routes;
use crate::servers;
use crate::status;


pub static EXTERNAL_FOLDER: &str = "./external_files";
pub static SHIPPED_FOLDER: &str = "./shipped_plugins";
pub static EXTERNAL_PLUGIN_FOLDER: &str = "./external_files/plugins";

pub static ENV_FILENAME: &str = "./external_files/.env";
pub static ENV_EXAMPLE_FILENAME: &str = "./.env.example";

pub static DB_FILENAME: &str = "./external_files/server-manager.db";

#[actix_web::main]
pub async fn start() -> std::io::Result<()> {
    start_scheduled_jobs().await;
    one_time_init()?;
    load_env_file();
    init_config();

    let bind_address = inmemory::get_config().get_string("bind_address").unwrap();

    let neccessary_migrations = migrations::check_necessary_migration(); // needs to be checked before db connection is done
    migrations::execute_pre_db_startup_migrations(&neccessary_migrations);

    let app_data = create_common_app_data();
    one_time_post_db_startup(&app_data).await;

    
    migrations::execute_post_db_startup_migrations(&neccessary_migrations, &app_data).await;
    migrations::save_migration(&neccessary_migrations, &app_data.app_data_persistence).await;
    
    init_server_list(&app_data.app_data_persistence).await;
    init_config_post_db(&app_data.app_data_persistence).await;
    
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

async fn init_server_list(persistence: &Persistence) {
   let servers =  servers::load_all_servers(persistence, false).await.unwrap();

    inmemory::insert_servers(servers);
}

async fn start_scheduled_jobs() {
    let scheduler = JobScheduler::new().await.unwrap();

    scheduler
        .add(
            Job::new("1/10 * * * * *", |_uuid, _l| {
                init_cache_silent();
            })
            .unwrap(),
        )
        .await
        .unwrap();
    scheduler
        .add(
            Job::new_async("1/5 * * * * *", |_uuid, _l| {
                Box::pin(async {
                    status::status_check_all().await;
                })
            })
            .unwrap(),
        )
        .await
        .unwrap();
    scheduler.add(
        Job::new_async("1/10 * * * * *", |_uuid, _l| {
            Box::pin(async {
                    features::check_all_main_conditions().await;
            })
        })
        .unwrap(),

    ) .await
    .unwrap();

    match scheduler.start().await {
        Ok(_res) => log::info!("Schedulder started"),
        Err(err) => log::error!("Could not start schedulder due to : {}", err),
    }
}

fn create_common_app_data() -> AppData {
    let persistence = futures::executor::block_on(create_persistence());
    let template_engine = create_templateengine();

    AppData {
        app_data_persistence: persistence,
        app_data_template_engine: template_engine,
    }
}

async fn create_persistence() -> Persistence {
    let db_url = format!("sqlite:{}?mode=rwc", DB_FILENAME);
    persistence::Persistence::new(&db_url).await
}

fn create_templateengine() -> handlebars::Handlebars<'static> {
    let config = inmemory::get_config();
    let template_base_path = config.get_string("template_base_path").unwrap();

    log::debug!("dir: {}", template_base_path);

    create_and_configure_template_engine(&template_base_path)
}

fn one_time_init() -> std::io::Result<()> {
    copy_files_into_external_folder()?;

    Ok(())
}

async fn one_time_post_db_startup(data: &AppData) {
    generate_encryption_key(data).await;
}

fn load_env_file() {
    dotenvy::from_path(Path::new(ENV_FILENAME)).ok();
}

async fn generate_encryption_key(data: &AppData) {
    data.app_data_persistence
        .insert(
            "encryption",
            Entry {
                key: "default".to_string(),
                value: crypt::get_random_key32().unwrap(),
            },
        )
        .await
        .unwrap();
}

/*due to how docker works, the external_folder that can be mapped to a local file, cannot be filled on startup, otherwise, the host folder will overlay the container folder
 => needs to be empty first and when started, we copy the content from another location in the external folder and make the content therefore also available on the docker host
*/
fn copy_files_into_external_folder() -> std::io::Result<()> {
    if !Path::new(EXTERNAL_PLUGIN_FOLDER).exists() {
        let src = Path::new(SHIPPED_FOLDER);
        let dst = Path::new(EXTERNAL_FOLDER);
        copy_dir_all(src, dst)?;
    }
    let env_file_path = Path::new(ENV_FILENAME);

    if !env_file_path.exists() {
        std::fs::copy(Path::new(ENV_EXAMPLE_FILENAME), env_file_path)?;
    }

    Ok(())
}

fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> std::io::Result<()> {
    std::fs::create_dir_all(&dst)?;
    for entry in std::fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            std::fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}

fn create_and_configure_template_engine(
    template_base_path: &str,
) -> handlebars::Handlebars<'static> {
    let mut handlebars = handlebars::Handlebars::new();
    handlebars
        .register_templates_directory(".html", template_base_path)
        .unwrap();
    handlebars.set_dev_mode(true);

    let templates = handlebars.get_templates();
    for entry in templates {
        log::debug!("registered template: {}", entry.0);
    }

    handlebars.set_strict_mode(true);
    handlebars.register_escape_fn(no_escape); //html escaping is the default and cause issue
    handlebars_helper_functions::register(&mut handlebars);

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
    handle_named_file(
        format!(
            "./server/static/assets/svg/{}",
            path.as_os_str().to_str().unwrap()
        )
        .as_str(),
    )
}

fn handle_named_file(file: &str) -> std::result::Result<fs::NamedFile, actix_web::Error> {
    let path_found = fs::NamedFile::open(file);
    match path_found {
        Ok(file) => Ok(file),
        Err(err) => {
            log::error!("File not found {}. Error was: {}", file, err);
            Err(actix_web::error::ErrorNotFound(""))
        }
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
    cfg.route(
        "/{filename:3rdpartylicenses.txt}",
        web::get().to(named_file),
    );

    cfg.route(
        "/assets/svg/{filename:.*\\.svg}",
        web::get().to(named_file_svg),
    );
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

    cfg.service(routes::get_system_dnsservers);
}

fn init_config() {
    let config = Config::builder()
        .add_source(config::Environment::default())
        .build()
        .expect("Could not load config from env properties"); // ok to panic, if the config cannot be loaded


    inmemory::set_config(config);    
    env_logger::init();

    if let Ok(number) = plugins::init_cache() {
        log::debug!("Loaded {} plugins into cache", number);
    };
}

async fn init_config_post_db(persistence: &Persistence) {
    let crypto_key = persistence.get("encryption", "default").await.map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e)).unwrap().ok_or(std::io::Error::new(std::io::ErrorKind::Other, "No crypto key in db found")).unwrap().value;
    
    inmemory::set_crypto_key(crypto_key);
}
