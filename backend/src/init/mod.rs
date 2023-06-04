mod files;
mod scheduling;
mod template_engine;

use config::Config;
use std::path::Path;

use crate::datastore::Persistence;
use crate::datastore::{self, TimeSeriesPersistence};
use crate::migrations;
use crate::models::error::AppError;
use crate::webserver;
use crate::webserver::AppData;

pub static ENV_FILENAME: &str = "./external_files/.env";

pub static DB_FILENAME: &str = "./external_files/server-manager.db";

pub async fn start() -> Result<(), AppError> {
    scheduling::start_scheduled_jobs().await;
    one_time_init()?;
    load_env_file();
    init_config();

    let bind_address = datastore::get_config().get_string("bind_address").unwrap();

    let neccessary_migrations = migrations::check_necessary_migration(); // needs to be checked before db connection is done
    migrations::execute_pre_db_startup_migrations(&neccessary_migrations);

    let app_data = create_common_app_data();
    one_time_post_db_startup(&app_data).await;

    migrations::execute_post_db_startup_migrations(&neccessary_migrations, &app_data).await;
    migrations::save_migration(&neccessary_migrations, &app_data.app_data_persistence).await;

    init_server_list(&app_data.app_data_persistence).await;
    init_config_post_db(&app_data.app_data_persistence).await;

    webserver::start_webserver(bind_address, app_data).await
}

async fn init_server_list(persistence: &Persistence) {
    let servers = datastore::load_all_servers(persistence, false)
        .await
        .unwrap();

    datastore::cache_servers(servers);
}

fn create_common_app_data() -> AppData {
    let persistence = futures::executor::block_on(create_persistence());
    let timeseries_persistence = futures::executor::block_on(create_timeseries_persistence());
    let template_engine = template_engine::create_templateengine();

    AppData {
        app_data_persistence: persistence,
        app_data_timeseries_persistence: timeseries_persistence,
        app_data_template_engine: template_engine,
    }
}

async fn create_persistence() -> Persistence {
    let db_url = format!("sqlite:{}?mode=rwc", DB_FILENAME);
    Persistence::new(&db_url).await
}

async fn create_timeseries_persistence() -> TimeSeriesPersistence {
    TimeSeriesPersistence::new().await
}

fn one_time_init() -> Result<(), AppError> {
    files::copy_files_into_external_folder()?;

    Ok(())
}

pub async fn one_time_post_db_startup(data: &AppData) {
    datastore::insert_new_encryption_key(&data.app_data_persistence).await;
}

fn load_env_file() {
    dotenvy::from_path(Path::new(ENV_FILENAME)).ok();
}

fn init_config() {
    let config = Config::builder()
        .add_source(config::Environment::default())
        .build()
        .expect("Could not load config from env properties"); // ok to panic, if the config cannot be loaded

    datastore::set_config(config);
    env_logger::init();
    datastore::init_cache();
}

async fn init_config_post_db(persistence: &Persistence) {
    let crypto_key = persistence
        .get("encryption", "default")
        .await
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
        .unwrap()
        .ok_or(std::io::Error::new(
            std::io::ErrorKind::Other,
            "No crypto key in db found",
        ))
        .unwrap()
        .value;

    datastore::set_crypto_key(crypto_key);
}
