use sqlx::FromRow;

pub mod migration;


#[derive(FromRow)]
pub struct Entry {
    pub key: String,
    pub value: String
}

#[derive(FromRow)]
pub struct Table {
    pub name: String
}