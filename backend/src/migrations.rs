use core::fmt;
use std::{path::Path, collections::HashMap};

use crate::{persistence::{Persistence, Migration}, init, appdata::AppData, servers,server_types::{Server, Credential, Feature}, crypt, plugin_types::Plugin, inmemory};

#[derive (PartialEq, Eq, Debug)]
pub enum MigrationTypes {
    DbLocation,
    Encryption
}

impl fmt::Display for MigrationTypes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MigrationTypes::DbLocation => write!(f, "DB_LOCATION"),
            MigrationTypes::Encryption=> write!(f, "ENCRYPTION")
        }
    }
}


pub fn check_necessary_migration() -> Vec<MigrationTypes> {
    let mut migrations: Vec<MigrationTypes> = Vec::new();

    let old_path = Path::new("./server-manager.db");
    let new_path = Path::new(init::DB_FILENAME);
    if old_path.exists() && ! new_path.exists() {       
        migrations.push(MigrationTypes::DbLocation);
        migrations.push(MigrationTypes::Encryption);
    }
   
    migrations
}

pub fn do_db_location_migration() -> std::result::Result<u64, std::io::Error> {
    let old_path = Path::new("./server-manager.db");
    let new_path = Path::new(init::DB_FILENAME);

    std::fs::copy(old_path,  new_path)
}


pub async fn save_migration(neccessary_migrations: &[MigrationTypes], persistence: &Persistence)  {
    let migrations: Vec<Migration> = neccessary_migrations.iter().map( |mig| Migration::new(mig.to_string().as_str())).collect();
    persistence.save_migrations(migrations).await.unwrap();
}

 pub async fn do_encryption_migration(data: &AppData) -> std::result::Result<(), std::io::Error>{
    match servers::load_all_servers(&data.app_data_persistence).await {
        Ok(servers) => {
            let plugins_map = inmemory::get_all_plugins_map();

            if !servers.is_empty() {
                let servers_data_to_encrypt: Vec<&Server> = servers.iter().filter(|server| server_needs_encryption(server, &plugins_map)).collect();

                log::info!("server that need encryption: {} ", servers_data_to_encrypt.len() );
                

                let crypto_key_entry = data.app_data_persistence.get("encryption", "default").await.map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?.ok_or(std::io::Error::new(std::io::ErrorKind::Other, "No crypto key in db found"))?;

               
                for server in servers_data_to_encrypt {
                    let mut s = server.to_owned();

                    for mut feature in &mut s.features {
                        let plugin = plugins_map.get(&feature.id.clone()).unwrap();

                        let mut new_credentials: Vec<Credential> = Vec::new();
                        for credential in &feature.credentials {
                            let credential_def = plugin.credentials.iter().find(|c| c.name == credential.name).unwrap().to_owned();

                            if credential_def.encrypt {
                                log::info!("will encrypt {} {}", feature.id, credential.name);
                                
                                new_credentials.push(Credential { name: credential.name.clone(), encrypted: true, value: crypt::default_encrypt(&credential.value, &crypto_key_entry.value) });
                            }
                            else {
                                log::info!("will not encrypt {} {}", feature.id, credential.name);
                                new_credentials.push(credential.to_owned());
                            }

                            }
                        
                        feature.credentials = new_credentials;
                    }
                    
                    update_server( &s, &data.app_data_persistence);
                }   
            }            
        },
        Err(err) => {
            log::error!("Error while loading servers: {}", err);
        }
    }
    Ok(())
}



fn update_server(server: &Server, persistence: &Persistence)  {
    futures::executor::block_on(
        servers::update_server(persistence, server)
    ).unwrap();    
}

fn server_needs_encryption(server: &Server, plugins_map: &HashMap<String, Plugin>) -> bool {
    server.features.iter().any(|f| feature_needs_encryption(f, plugins_map))
}
fn feature_needs_encryption(feature: &Feature, plugins_map: &HashMap<String, Plugin>) -> bool {
    feature.credentials.iter().any(|c| plugins_map.get(&feature.id).unwrap().credentials.iter().any(|cdef| cdef.name == c.name && cdef.encrypt))
}

pub fn execute_pre_db_startup_migrations(neccessary_migrations: &[MigrationTypes]) {
    if neccessary_migrations.contains(&MigrationTypes::DbLocation) {
        do_db_location_migration().unwrap();
    }
}

pub async fn execute_post_db_startup_migrations(neccessary_migrations:&[MigrationTypes], data: &AppData) {
    if neccessary_migrations.contains(&MigrationTypes::Encryption) {
        do_encryption_migration(data).await.unwrap();
    }
}

