use rusqlite::{Connection, Result};

#[allow(unused)]
pub struct SqliteDatabase {
    url: String,
    connection: Connection,
}

impl SqliteDatabase {
    #[allow(unused)]
    pub fn new(url: &str) -> Result<SqliteDatabase> {
        Ok(SqliteDatabase {
            url: url.to_string(),
            connection: Connection::open(url)?,
        })
    }
}

//TODO: impl Database for SqliteDatabase {}
