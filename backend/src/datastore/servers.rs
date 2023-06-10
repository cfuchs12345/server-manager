use std::net::IpAddr;

use crate::{
    common,
    models::{
        error::AppError,
        plugin::Plugin,
        server::{Credential, Feature, Server},
    },
};

use super::{inmemory, persistence::Persistence, Entry};

const TABLE: &str = "servers";

fn json_to_server(json: &str) -> Result<Server, AppError> {
    serde_json::from_str(json).map_err(AppError::from)
}

fn entries_to_servers(entries: Vec<Entry>) -> Result<Vec<Server>, AppError> {
    let mut list = Vec::new();
    for entry in entries {
        list.push(json_to_server(&entry.value)?);
    }
    Ok(list)
}

fn server_to_entry(server: &Server) -> Result<Entry, AppError> {
    Ok(Entry {
        key: format!("{}", server.ipaddress),
        value: serde_json::to_string(server)?,
    })
}

pub async fn insert_server(persistence: &Persistence, server: &Server) -> Result<bool, AppError> {
    let crypto_key = super::get_crypto_key()?;

    let encrypted_server = de_or_encrypt_fields(
        server,
        common::default_encrypt,
        credential_needs_encryption,
        crypto_key.as_str(),
    )
    .await?;
    log::info!("Server after encryption: {:?}", encrypted_server);

    inmemory::add_server(&encrypted_server)?;
    let result = persistence
        .insert(TABLE, server_to_entry(&encrypted_server)?)
        .await?;

    Ok(result > 0)
}

pub async fn update_server(persistence: &Persistence, server: &Server) -> Result<bool, AppError> {
    let crypto_key = super::get_crypto_key()?;

    let encrypted_server = de_or_encrypt_fields(
        server,
        common::default_encrypt,
        credential_needs_encryption,
        crypto_key.as_str(),
    )
    .await?;

    log::info!("Server after encryption: {:?}", encrypted_server);
    inmemory::add_server(&encrypted_server)?;
    let result = persistence
        .update(TABLE, server_to_entry(&encrypted_server)?)
        .await?;

    Ok(result > 0)
}

pub async fn delete_server(
    persistence: &Persistence,
    ipaddress: &IpAddr,
) -> Result<bool, AppError> {
    inmemory::remove_server(ipaddress)?;
    let result = persistence
        .delete(TABLE, format!("{}", ipaddress).as_str())
        .await?;

    Ok(result > 0)
}

pub async fn get_all_servers(
    persistence: &Persistence,
    use_cache: bool,
) -> Result<Vec<Server>, AppError> {
    if use_cache {
        inmemory::get_all_servers()
    } else {
        let server_entries = persistence
            .get_all(TABLE, Some("inet_aton(key) asc"))
            .await?;

        Ok(entries_to_servers(server_entries)?)
    }
}

pub async fn get_server(persistence: &Persistence, ipaddress: &IpAddr) -> Result<Server, AppError> {
    let opt = persistence
        .get(TABLE, format!("{}", ipaddress).as_str())
        .await?;
    match opt {
        Some(entry) => Ok(json_to_server(&entry.value)?),
        None => Err(AppError::ServerNotFound(format!("{}", ipaddress))),
    }
}

pub async fn re_encrypt_servers(
    servers: Vec<Server>,
    password_for_encryption: &str,
    direction_is_out: bool,
) -> Result<Vec<Server>, AppError> {
    let mut updated_servers = Vec::new();
    let default_crypto_key = super::get_crypto_key()?;

    let decrypt_key = match direction_is_out {
        true => default_crypto_key.clone(),
        false => password_for_encryption.to_owned(),
    };

    let encrypt_key = match direction_is_out {
        true => password_for_encryption.to_owned(),
        false => default_crypto_key,
    };

    for server in servers {
        if !could_need_encryption(&server)? {
            updated_servers.push(server);
            continue;
        }

        let decrypted = de_or_encrypt_fields(
            &server,
            common::default_encrypt,
            credential_needs_decryption,
            decrypt_key.as_str(),
        )
        .await?;
        let encrypted = de_or_encrypt_fields(
            &decrypted,
            common::default_encrypt,
            credential_needs_encryption,
            encrypt_key.as_str(),
        )
        .await?;

        updated_servers.push(encrypted);
    }
    Ok(updated_servers)
}

pub async fn de_or_encrypt_fields(
    server: &Server,
    crypt_func: fn(&str, &str) -> String,
    check_func: fn(&Credential, &Plugin) -> bool,
    crypto_key: &str,
) -> Result<Server, AppError> {
    if !could_need_encryption(server)? {
        return Ok(server.clone());
    }

    let mut server_result = server.clone();

    for feature in &server.features {
        if let Some(plugin) = super::get_plugin(feature.id.as_str())? {
            if !has_encrypted_fields(&plugin) {
                continue;
            }
            let de_or_encrypted_credentials = de_or_encrypted_credentials(
                feature, plugin, server, crypto_key, crypt_func, check_func,
            );
            server_result = update_feature(server_result, feature, de_or_encrypted_credentials);
        }
    }

    Ok(server_result)
}

fn could_need_encryption(server: &Server) -> Result<bool, AppError> {
    if server.features.is_empty() {
        Ok(false)
    } else {
        Ok(server
            .features
            .iter()
            .map(|feature| feature_could_need_encryption(feature).unwrap_or(false))
            .any(|b| b))
    }
}

fn feature_could_need_encryption(feature: &Feature) -> Result<bool, AppError> {
    if let Some(plugin) = super::get_plugin(feature.id.as_str())? {
        Ok(has_encrypted_fields(&plugin))
    } else {
        Ok(false)
    }
}

fn de_or_encrypted_credentials(
    feature: &crate::models::server::Feature,
    plugin: Plugin,
    server: &Server,
    key: &str,
    crypt_func: fn(&str, &str) -> String,
    check_func: fn(&Credential, &Plugin) -> bool,
) -> Vec<Credential> {
    let mut new_credentials = Vec::new();
    for credential in &feature.credentials {
        let mut clone_credential = credential.clone();

        if check_func(credential, &plugin) {
            log::debug!(
                "credential {:?} for server {} needs encryption",
                credential,
                server.ipaddress
            );
            clone_credential.encrypted = !clone_credential.encrypted;
            clone_credential.value = crypt_func(credential.value.as_str(), key);
        }
        new_credentials.push(clone_credential);
    }
    new_credentials
}

fn update_feature(
    mut server: Server,
    feature: &crate::models::server::Feature,
    new_credentials: Vec<Credential>,
) -> Server {
    if let Some(mut clone_feature) = server.find_feature(&feature.id) {
        clone_feature.credentials = new_credentials;
        server = server.replace_feature(clone_feature);
    }
    server
}

fn has_encrypted_fields(plugin: &Plugin) -> bool {
    plugin
        .credentials
        .iter()
        .any(|credential_definition| credential_definition.encrypt)
}

fn credential_needs_decryption(credential: &Credential, _plugin: &Plugin) -> bool {
    credential.encrypted
}

fn credential_needs_encryption(credential: &Credential, plugin: &Plugin) -> bool {
    if credential.encrypted {
        false
    } else {
        plugin.credentials.iter().any(|credential_definition| {
            credential_definition.name == credential.name && credential_definition.encrypt
        })
    }
}
