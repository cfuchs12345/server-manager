use core::fmt;
use std::{collections::HashMap, path::Path};

use crate::{
    common,
    datastore::{self, Migration},
    models::{
        error::AppError,
        plugin::Plugin,
        server::{Credential, Feature, Server},
    },
};

#[derive(PartialEq, Eq, Debug)]
pub enum MigrationTypes {
    DbLocation,
    Encryption,
}

impl fmt::Display for MigrationTypes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MigrationTypes::DbLocation => write!(f, "DB_LOCATION"),
            MigrationTypes::Encryption => write!(f, "ENCRYPTION"),
        }
    }
}

pub fn check_necessary_migration() -> Vec<MigrationTypes> {
    let mut migrations: Vec<MigrationTypes> = Vec::new();

    let old_path = Path::new("./server-manager.db");
    let new_path = Path::new(common::DB_FILENAME);
    if old_path.exists() && !new_path.exists() {
        migrations.push(MigrationTypes::DbLocation);
        migrations.push(MigrationTypes::Encryption);
    }

    migrations
}

pub fn do_db_location_migration() -> std::result::Result<u64, AppError> {
    let old_path = Path::new("./server-manager.db");
    let new_path = Path::new(common::DB_FILENAME);

    std::fs::copy(old_path, new_path).map_err(|e| AppError::Unknown(format!("{}", e)))
}

pub async fn save_migration(neccessary_migrations: &[MigrationTypes]) -> Result<(), AppError> {
    let migrations: Vec<Migration> = neccessary_migrations
        .iter()
        .map(|mig| Migration::new(mig.to_string().as_str()))
        .collect();
    datastore::save_migrations(migrations).await?;
    Ok(())
}

pub async fn do_encryption_migration() -> std::result::Result<(), AppError> {
    let servers = datastore::get_all_servers(false).await?;

    let plugins_map = datastore::get_all_plugins_map()?;

    if !servers.is_empty() {
        let servers_data_to_encrypt = get_servers_needing_encryption(servers, &plugins_map)?;

        let crypto_key_entry = get_default_encryption_key().await?;

        for server in servers_data_to_encrypt {
            let mut s = server.to_owned();

            for mut feature in &mut s.features {
                let plugin =
                    plugins_map
                        .get(&feature.id.clone())
                        .ok_or(AppError::UnknownPlugin(format!(
                            "plugin {} not found",
                            feature.id
                        )))?;

                let mut new_credentials: Vec<Credential> = Vec::new();
                for credential in &feature.credentials {
                    let credential_def = plugin
                        .credentials
                        .iter()
                        .find(|c| c.name == credential.name)
                        .ok_or(AppError::CredentialNotFound(format!(
                            "Credential {} not found",
                            credential.name
                        )))?
                        .to_owned();

                    if credential_def.encrypt {
                        log::debug!("will encrypt {} {}", feature.id, credential.name);

                        new_credentials.push(Credential {
                            name: credential.name.clone(),
                            encrypted: true,
                            value: common::default_encrypt(
                                &credential.value,
                                &crypto_key_entry.value,
                            )?,
                        });
                    } else {
                        new_credentials.push(credential.to_owned());
                    }
                }

                feature.credentials = new_credentials;
            }

            update_server(&s)?;
        }
    }
    Ok(())
}

async fn get_default_encryption_key() -> Result<datastore::Entry, AppError> {
    let crypto_key_entry = datastore::get_encryption_key()
        .await?
        .ok_or_else(|| AppError::DataNotFound("encryption/default".to_string()))?;
    Ok(crypto_key_entry)
}

fn get_servers_needing_encryption(
    servers: Vec<Server>,
    plugins_map: &HashMap<String, Plugin>,
) -> Result<Vec<Server>, AppError> {
    let mut list = Vec::new();
    for server in servers {
        if server_needs_encryption(&server, plugins_map)? {
            list.push(server);
        }
    }
    Ok(list)
}

fn update_server(server: &Server) -> Result<(), AppError> {
    futures::executor::block_on(datastore::update_server(server))?;
    Ok(())
}

fn server_needs_encryption(
    server: &Server,
    plugins_map: &HashMap<String, Plugin>,
) -> Result<bool, AppError> {
    for feature in &server.features {
        if feature_needs_encryption(feature, plugins_map)? {
            return Ok(true);
        }
    }
    Ok(false)
}

fn feature_needs_encryption(
    feature: &Feature,
    plugins_map: &HashMap<String, Plugin>,
) -> Result<bool, AppError> {
    let plugin = plugins_map
        .get(&feature.id)
        .ok_or(AppError::UnknownPlugin(format!(
            "Plugin {} not found",
            feature.id
        )))?;

    Ok(feature.credentials.iter().any(|c| {
        plugin
            .credentials
            .iter()
            .any(|cdef| cdef.name == c.name && cdef.encrypt)
    }))
}

pub fn execute_pre_db_startup_migrations(
    neccessary_migrations: &[MigrationTypes],
) -> Result<(), AppError> {
    if neccessary_migrations.contains(&MigrationTypes::DbLocation) {
        do_db_location_migration()?;
    }

    Ok(())
}

pub async fn execute_post_db_startup_migrations(
    neccessary_migrations: &[MigrationTypes],
) -> Result<(), AppError> {
    if neccessary_migrations.contains(&MigrationTypes::Encryption) {
        do_encryption_migration().await?;
    }

    Ok(())
}
