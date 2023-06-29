use futures_util::lock::Mutex;
use lazy_static::lazy_static;
use sqlx::{sqlite::SqliteConnectOptions, ConnectOptions, Pool, Sqlite, SqlitePool};
use std::str::FromStr;

use crate::{common, models::error::AppError};

use super::{Entry, Migration};

lazy_static! {
    static ref PERSISTENCE: Mutex<Persistence> = {
        let db_url = format!("sqlite:{}?mode=rwc", common::DB_FILENAME);

        Mutex::new(Persistence::new(&db_url))
    };
}

#[derive(Debug, Clone)]
struct Persistence {
    pool: Pool<Sqlite>,
}

impl Persistence {
    pub fn new(db_url: &str) -> Self {
        log::debug!("DB URL = {}", db_url);

        Persistence {
            pool: Self::get_connection(db_url),
        }
    }

    fn get_connection(db_url: &str) -> Pool<Sqlite> {
        let mut options = SqliteConnectOptions::from_str(db_url)
            .expect("Could not get db url")
            .extension("inet")
            .create_if_missing(true);

        let new_opts = options.disable_statement_logging();

        SqlitePool::connect_lazy_with(new_opts.to_owned())
    }

    pub async fn _close(self) {
        self.pool.close().await;
    }
}

pub async fn init_db() -> Result<(), AppError> {
    create_tables(vec![
        ("migration", vec![("date", "DATETIME"), ("name", "TEXT")]),
        ("encryption", vec![("key", "TEXT"), ("value", "TEXT")]),
        ("servers", vec![("key", "TEXT"), ("value", "TEXT")]),
        ("users", vec![("key", "TEXT"), ("value", "TEXT")]),
        ("plugin_config", vec![("key", "TEXT"), ("value", "TEXT")]),
        ("dns_servers", vec![("key", "TEXT"), ("value", "TEXT")]),
        ("notifications", vec![("key", "TEXT"), ("value", "TEXT")]),
        ("alarms", vec![("key", "TEXT"), ("value", "TEXT")]),
    ])
    .await?;

    create_index(vec![("servers", true, vec!["key"])]).await?;
    create_index(vec![("users", true, vec!["key"])]).await?;
    create_index(vec![("plugin_config", true, vec!["key"])]).await?;
    create_index(vec![("dns_servers", true, vec!["key"])]).await?;
    create_index(vec![("notifications", true, vec!["key"])]).await?;
    create_index(vec![("alarms", true, vec!["key"])]).await?;
    Ok(())
}

pub async fn create_tables(tuples: Vec<(&str, Vec<(&str, &str)>)>) -> Result<(), AppError> {
    // transaction shouldn't be necessary, but doesn't cost much...
    let mut transaction = PERSISTENCE.lock().await.pool.begin().await?;

    for tuple in tuples {
        let create = get_create_statement(tuple.0, tuple.1);

        sqlx::query(create.as_str())
            .execute(&mut transaction)
            .await?;
    }

    transaction.commit().await?;
    Ok(())
}

pub async fn create_index(tuples: Vec<(&str, bool, Vec<&str>)>) -> Result<(), AppError> {
    // transaction shouldn't be necessary, but doesn't cost much...
    let mut transaction: sqlx::Transaction<'_, Sqlite> =
        PERSISTENCE.lock().await.pool.begin().await?;

    for tuple in tuples {
        let create = get_create_index_statement(tuple.0, tuple.1, tuple.2);

        sqlx::query(create.as_str())
            .execute(&mut transaction)
            .await?;
    }

    transaction.commit().await?;
    Ok(())
}

#[allow(dead_code)]
pub async fn get_migration(name: &str) -> Result<Migration, AppError> {
    let mut transaction = PERSISTENCE.lock().await.pool.begin().await?;

    let select = "SELECT * from migration where name = ?";
    let result: Migration = sqlx::query_as(select)
        .bind(name)
        .fetch_one(&mut transaction)
        .await?;

    transaction.commit().await?;
    Ok(result)
}

#[allow(dead_code)]
pub async fn save_migration(migration: Migration) -> Result<u64, AppError> {
    let mut transaction = PERSISTENCE.lock().await.pool.begin().await?;

    let insert = "INSERT INTO migration values (?, ?)";

    let result = sqlx::query(insert)
        .bind(migration.name)
        .bind(migration.date)
        .execute(&mut transaction)
        .await?;
    transaction.commit().await?;
    Ok(result.rows_affected())
}

pub async fn save_migrations(migrations: Vec<Migration>) -> Result<u64, AppError> {
    let mut transaction = PERSISTENCE.lock().await.pool.begin().await?;

    let insert = "INSERT INTO migration values (?, ?)";
    let mut result: u64 = 0;

    for migration in migrations {
        result += sqlx::query(insert)
            .bind(migration.name)
            .bind(migration.date)
            .execute(&mut transaction)
            .await?
            .rows_affected();
    }

    transaction.commit().await?;
    Ok(result)
}

pub async fn get_all(table: &str, order_by: Option<&str>) -> Result<Vec<Entry>, AppError> {
    let mut transaction = PERSISTENCE.lock().await.pool.begin().await?;

    let select = get_select_all_statement(table, order_by); // inet_aton is a function from inet extension - converts the xxx.xxx.xxx.xxx in a numeric value so that it can be easily sorted

    let result: Vec<Entry> = sqlx::query_as(select.as_str())
        .fetch_all(&mut transaction)
        .await?;
    transaction.commit().await?;
    Ok(result)
}

pub async fn get(table: &str, key: &str) -> Result<Option<Entry>, AppError> {
    let mut transaction = PERSISTENCE.lock().await.pool.begin().await?;

    let select = get_select_statement(table); // inet_aton is a function from inet extension - converts the xxx.xxx.xxx.xxx in a numeric value so that it can be easily sorted

    let result: Option<Entry> = sqlx::query_as(select.as_str())
        .bind(key)
        .fetch_one(&mut transaction)
        .await
        .ok();
    transaction.commit().await?;
    Ok(result)
}

pub async fn update(table: &str, entry: Entry) -> Result<u64, AppError> {
    let mut transaction = PERSISTENCE.lock().await.pool.begin().await?;
    let update = get_update_statement(table);

    let result = sqlx::query(update.as_str())
        .bind(entry.value)
        .bind(entry.key)
        .execute(&mut transaction)
        .await?;
    transaction.commit().await?;
    Ok(result.rows_affected())
}

pub async fn delete(table: &str, key: &str) -> Result<u64, AppError> {
    let mut transaction = PERSISTENCE.lock().await.pool.begin().await?;
    let delete = get_delete_statement(table);

    let result = sqlx::query(delete.as_str())
        .bind(key)
        .execute(&mut transaction)
        .await?;
    transaction.commit().await?;
    Ok(result.rows_affected())
}

pub async fn insert(table: &str, entry: Entry) -> Result<u64, AppError> {
    let mut transaction = PERSISTENCE.lock().await.pool.begin().await?;

    let insert = get_insert_statement(table);

    let result = sqlx::query(insert.as_str())
        .bind(entry.key)
        .bind(entry.value)
        .execute(&mut transaction)
        .await?;
    transaction.commit().await?;
    Ok(result.rows_affected())
}

fn get_create_statement(table: &str, columns: Vec<(&str, &str)>) -> String {
    let columns_tmp: Vec<String> = columns
        .iter()
        .map(|tuple| tuple.0.to_string() + " " + tuple.1)
        .collect();
    let columns_as_str = columns_tmp.join(", ");

    format!("CREATE TABLE IF NOT EXISTS {} ({})", table, columns_as_str)
}

fn get_create_index_statement(table: &str, unique: bool, columns: Vec<&str>) -> String {
    let unique_str = match unique {
        true => "UNIQUE",
        false => "",
    };
    format!(
        "CREATE {} INDEX IF NOT EXISTS IDX_{} ON {} ({})",
        unique_str,
        table,
        table,
        columns.join(", ")
    )
}

fn get_select_all_statement(table: &str, order_by: Option<&str>) -> String {
    let str_to_add = match order_by {
        Some(value) => format!(" ORDER BY {}", value),
        None => "".to_string(),
    };
    format!("SELECT key, value FROM {}{}", table, str_to_add)
}

fn get_select_statement(table: &str) -> String {
    format!("SELECT key, value FROM {} WHERE key = ?", table)
}

fn get_update_statement(table: &str) -> String {
    format!("UPDATE {} set value = ? WHERE key = ?", table)
}

fn get_insert_statement(table: &str) -> String {
    format!("INSERT INTO {} VALUES( ?, ?)", table)
}

fn get_delete_statement(table: &str) -> String {
    format!("DELETE FROM {} WHERE key = ?", table)
}
