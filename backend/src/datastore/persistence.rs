use sqlx::{sqlite::SqliteConnectOptions, ConnectOptions, Pool, Sqlite, SqlitePool};
use std::str::FromStr;

use crate::{common, models::error::AppError};

use super::{Entry, Migration};

#[derive(Debug, Clone)]
pub struct Persistence {
    pool: Pool<Sqlite>,
}

impl Persistence {
    pub async fn new(db_url: &str) -> Result<Persistence, AppError> {
        log::debug!("DB URL = {}", db_url);

        let instance = Persistence {
            pool: Self::get_connection(db_url)
                .await
                .unwrap_or_else(|err| panic!("Cannot connect to database: {}, {}", db_url, err)),
        };
        instance
            .create_tables(vec![
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
        instance
            .create_index(vec![("servers", true, vec!["key"])])
            .await?;
        instance
            .create_index(vec![("users", true, vec!["key"])])
            .await?;
        instance
            .create_index(vec![("plugin_config", true, vec!["key"])])
            .await?;
        instance
            .create_index(vec![("dns_servers", true, vec!["key"])])
            .await?;
        instance
            .create_index(vec![("notifications", true, vec!["key"])])
            .await?;
        instance
            .create_index(vec![("alarms", true, vec!["key"])])
            .await?;
        Ok(instance)
    }

    pub async fn get_instance() -> Result<Persistence, AppError> {
        let db_url = format!("sqlite:{}?mode=rwc", common::DB_FILENAME);
        Persistence::new(&db_url).await
    }

    async fn get_connection(db_url: &str) -> Result<Pool<Sqlite>, AppError> {
        let mut options = SqliteConnectOptions::from_str(db_url)?
            .extension("inet")
            .create_if_missing(true);

        let new_opts = options.disable_statement_logging();

        SqlitePool::connect_with(new_opts.to_owned())
            .await
            .map_err(|err| {
                AppError::DatabaseError(format!("Could not connect to sqlite. Error was: {}", err))
            })
    }

    pub async fn _close(self) {
        self.pool.close().await;
    }

    pub async fn create_tables(
        &self,
        tuples: Vec<(&str, Vec<(&str, &str)>)>,
    ) -> Result<(), AppError> {
        // transaction shouldn't be necessary, but doesn't cost much...
        let mut transaction = self.pool.begin().await?;

        for tuple in tuples {
            let create = get_create_statement(tuple.0, tuple.1);

            sqlx::query(create.as_str())
                .execute(&mut transaction)
                .await?;
        }

        transaction.commit().await?;
        Ok(())
    }

    pub async fn create_index(&self, tuples: Vec<(&str, bool, Vec<&str>)>) -> Result<(), AppError> {
        // transaction shouldn't be necessary, but doesn't cost much...
        let mut transaction = self.pool.begin().await?;

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
    pub async fn get_migration(&self, name: &str) -> Result<Migration, AppError> {
        let mut transaction = self.pool.begin().await?;

        let select = "SELECT * from migration where name = ?";
        let result: Migration = sqlx::query_as(select)
            .bind(name)
            .fetch_one(&mut transaction)
            .await?;

        transaction.commit().await?;
        Ok(result)
    }

    #[allow(dead_code)]
    pub async fn save_migration(&self, migration: Migration) -> Result<u64, AppError> {
        let mut transaction = self.pool.begin().await?;

        let insert = "INSERT INTO migration values (?, ?)";

        let result = sqlx::query(insert)
            .bind(migration.name)
            .bind(migration.date)
            .execute(&mut transaction)
            .await?;
        transaction.commit().await?;
        Ok(result.rows_affected())
    }

    pub async fn save_migrations(&self, migrations: Vec<Migration>) -> Result<u64, AppError> {
        let mut transaction = self.pool.begin().await?;

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

    pub async fn get_all(
        &self,
        table: &str,
        order_by: Option<&str>,
    ) -> Result<Vec<Entry>, AppError> {
        let mut transaction = self.pool.begin().await?;

        let select = get_select_all_statement(table, order_by); // inet_aton is a function from inet extension - converts the xxx.xxx.xxx.xxx in a numeric value so that it can be easily sorted

        let result: Vec<Entry> = sqlx::query_as(select.as_str())
            .fetch_all(&mut transaction)
            .await?;
        transaction.commit().await?;
        Ok(result)
    }

    pub async fn get(&self, table: &str, key: &str) -> Result<Option<Entry>, AppError> {
        let mut transaction = self.pool.begin().await?;

        let select = get_select_statement(table); // inet_aton is a function from inet extension - converts the xxx.xxx.xxx.xxx in a numeric value so that it can be easily sorted

        let result: Option<Entry> = sqlx::query_as(select.as_str())
            .bind(key)
            .fetch_one(&mut transaction)
            .await
            .ok();
        transaction.commit().await?;
        Ok(result)
    }

    pub async fn update(&self, table: &str, entry: Entry) -> Result<u64, AppError> {
        let mut transaction = self.pool.begin().await?;
        let update = get_update_statement(table);

        let result = sqlx::query(update.as_str())
            .bind(entry.value)
            .bind(entry.key)
            .execute(&mut transaction)
            .await?;
        transaction.commit().await?;
        Ok(result.rows_affected())
    }

    pub async fn delete(&self, table: &str, key: &str) -> Result<u64, AppError> {
        let mut transaction = self.pool.begin().await?;
        let delete = get_delete_statement(table);

        let result = sqlx::query(delete.as_str())
            .bind(key)
            .execute(&mut transaction)
            .await?;
        transaction.commit().await?;
        Ok(result.rows_affected())
    }

    pub async fn insert(&self, table: &str, entry: Entry) -> Result<u64, AppError> {
        let mut transaction = self.pool.begin().await?;

        let insert = get_insert_statement(table);

        let result = sqlx::query(insert.as_str())
            .bind(entry.key)
            .bind(entry.value)
            .execute(&mut transaction)
            .await?;
        transaction.commit().await?;
        Ok(result.rows_affected())
    }
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
