use sqlx::{FromRow, types::chrono::{NaiveDateTime, Utc}};


#[derive(FromRow)]
pub struct Migration {
    pub date: NaiveDateTime,
    pub name: String
}

impl Migration {
    pub fn new(name: &str) -> Self {
        Self { 
            date: Utc::now().naive_utc(),
            name: name.to_string()
         }
    }
}
