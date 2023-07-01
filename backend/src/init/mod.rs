mod files;
mod scheduling;
mod template_engine;

use config::Config;
use std::path::Path;

use crate::datastore::{self, TimeSeriesPersistence};
use crate::migrations;
use crate::models::error::AppError;
use crate::webserver;
use crate::webserver::AppData;

pub static ENV_FILENAME: &str = "./external_files/.env";

pub async fn start() -> Result<(), AppError> {
    scheduling::start_scheduled_jobs().await?;
    one_time_init()?;
    load_env_file();
    init_config()?;

    let bind_address = datastore::get_config()?.get_string("bind_address")?;

    let neccessary_migrations = migrations::check_necessary_migration(); // needs to be checked before db connection is done
    migrations::execute_pre_db_startup_migrations(&neccessary_migrations)?;

    let app_data = create_common_app_data()?;
    datastore::init_db().await?;
    one_time_post_db_startup().await?;

    migrations::execute_post_db_startup_migrations(&neccessary_migrations).await?;
    migrations::save_migration(&neccessary_migrations).await?;

    init_server_list().await?;
    init_config_post_db().await?;

    webserver::start_webserver(bind_address, app_data).await
}

async fn init_server_list() -> Result<(), AppError> {
    let servers = datastore::get_all_servers(false).await?;

    datastore::cache_servers(servers)?;
    Ok(())
}

fn create_common_app_data() -> Result<AppData, AppError> {
    let timeseries_persistence = futures::executor::block_on(create_timeseries_persistence())?;
    let template_engine = template_engine::create_templateengine()?;

    Ok(AppData {
        app_data_timeseries_persistence: timeseries_persistence,
        app_data_template_engine: template_engine,
    })
}

async fn create_timeseries_persistence() -> Result<TimeSeriesPersistence, AppError> {
    TimeSeriesPersistence::new().await
}

fn one_time_init() -> Result<(), AppError> {
    files::copy_files_into_external_folder()?;

    Ok(())
}
/* The methods called here should only do it's job once, like initializing the encryption key and so on
 */
pub async fn one_time_post_db_startup() -> Result<(), AppError> {
    datastore::insert_new_encryption_key().await?;

    Ok(())
}

fn load_env_file() {
    dotenvy::from_path(Path::new(ENV_FILENAME)).ok();
}

fn init_config() -> Result<(), AppError> {
    let config = Config::builder()
        .add_source(config::Environment::default())
        .build()?; // ok to panic, if the config cannot be loaded

    datastore::set_config(config)?;
    env_logger::init();
    datastore::init_cache();

    Ok(())
}

async fn init_config_post_db() -> Result<(), AppError> {
    let crypto_key = datastore::get_encryption_key()
        .await?
        .ok_or(AppError::Unknown(
            "Cryto key not found in database".to_owned(),
        ))?
        .value;

    datastore::set_crypto_key(crypto_key)
}
