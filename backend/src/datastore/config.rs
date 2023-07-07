use std::net::IpAddr;

use crate::{
    common,
    models::{
        config::{dns_server::DNSServer, Configuration},
        error::AppError,
        server::Server,
        users::User,
    },
};

use super::{persistence, Entry};

const TABLE_DNS_SERVERS: &str = "dns_servers";
const TABLE_ENCRYPTION: &str = "encryption";

fn json_to_dnsserver(json: &str) -> Result<DNSServer, AppError> {
    serde_json::from_str(json).map_err(AppError::from)
}

fn entries_to_dnsservers(jsons: Vec<Entry>) -> Result<Vec<DNSServer>, AppError> {
    let mut dns_servers = Vec::new();
    for entry in jsons {
        dns_servers.push(json_to_dnsserver(&entry.value)?);
    }
    Ok(dns_servers)
}

fn dnsserver_to_entry(dns_server: &DNSServer) -> Result<Entry, AppError> {
    Ok(Entry {
        key: format!("{}", dns_server.ipaddress),
        value: serde_json::to_string(dns_server)?,
    })
}

pub async fn insert_dnsserver(server: &DNSServer) -> Result<bool, AppError> {
    let result = persistence::insert(TABLE_DNS_SERVERS, dnsserver_to_entry(server)?).await?;

    Ok(result > 0)
}

pub async fn upate_dnsserver(server: &DNSServer) -> Result<bool, AppError> {
    let result = persistence::update(TABLE_DNS_SERVERS, dnsserver_to_entry(server)?).await?;

    Ok(result > 0)
}

pub async fn delete_dnsserver(ipaddress: &IpAddr) -> Result<bool, AppError> {
    let result = persistence::delete(TABLE_DNS_SERVERS, format!("{}", ipaddress).as_str()).await?;

    Ok(result > 0)
}

pub async fn get_all_dnsservers() -> Result<Vec<DNSServer>, AppError> {
    let server_entries =
        persistence::get_all(TABLE_DNS_SERVERS, Some("inet_aton(key) asc")).await?;

    entries_to_dnsservers(server_entries)
}

pub async fn get_encryption_key() -> Result<Option<Entry>, AppError> {
    let key = "default";

    persistence::get(TABLE_ENCRYPTION, key).await
}

pub async fn insert_new_encryption_key() -> Result<u64, AppError> {
    let key = "default";

    if get_encryption_key().await?.is_none() {
        log::info!("new encryption key saved in database");

        persistence::insert(
            TABLE_ENCRYPTION,
            Entry {
                key: key.to_owned(),
                value: common::get_random_key32()?,
            },
        )
        .await
        .map_err(AppError::from)
    } else {
        Ok(0)
    }
}

pub async fn export_config(password_for_encryption: &str) -> Result<Configuration, AppError> {
    let dns_servers = super::get_all_dnsservers().await?;
    let encrypted_users =
        super::encrypt_users(super::get_all_users(true).await?, password_for_encryption)?;
    let disabled_plugins = super::get_disabled_plugins().await?;
    let re_encrypted_servers = super::re_encrypt_servers(
        super::get_all_servers(false).await?,
        password_for_encryption,
        true,
    )?;

    Ok(Configuration {
        dns_servers,
        users: encrypted_users,
        disabled_plugins,
        servers: re_encrypted_servers,
    })
}

pub async fn import_config(
    config: Configuration,
    replace_existing: bool,
    password_for_decryption: &str,
) -> Result<bool, AppError> {
    let decrypted_users = super::decrypt_users(config.users, password_for_decryption)?;
    let decrypted_servers =
        super::re_encrypt_servers(config.servers, password_for_decryption, false)?;

    for dns_server in config.dns_servers {
        if dns_server_exists(&dns_server).await? {
            if replace_existing {
                super::upate_dnsserver(&dns_server).await?;
            }
        } else {
            super::insert_dnsserver(&dns_server).await?;
        }
    }

    super::disable_plugins(config.disabled_plugins).await?;

    for server in decrypted_servers {
        if server_exists(&server).await? {
            if replace_existing {
                super::update_server(&server).await?;
            }
        } else {
            super::insert_server(&server).await?;
        }
    }

    for user in decrypted_users {
        if user_exists(&user).await? {
            super::delete_user(user.get_user_id().as_str()).await?;
            super::insert_user(&user).await?;
        } else {
            super::insert_user(&user).await?;
        }
    }

    Ok(true)
}

async fn user_exists(user: &User) -> Result<bool, AppError> {
    let existing_users = super::get_all_users(false).await?;

    Ok(existing_users
        .iter()
        .any(|existing| existing.get_user_id() == user.get_user_id()))
}

async fn server_exists(server: &Server) -> Result<bool, AppError> {
    let existing_servers = super::get_all_servers(false).await?;

    Ok(existing_servers
        .iter()
        .any(|existing| existing.get_ipaddress() == server.get_ipaddress()))
}

async fn dns_server_exists(dns_server: &DNSServer) -> Result<bool, AppError> {
    let existing_dns_servers = super::get_all_dnsservers().await?;

    Ok(existing_dns_servers
        .iter()
        .any(|existing| existing.ipaddress == dns_server.ipaddress))
}
