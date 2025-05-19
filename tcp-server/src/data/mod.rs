pub mod database;
//#[cfg(not(feature = "sql"))]
pub mod mock_database;
pub mod sqlite_database;

pub use self::database::Database;
//#[cfg(not(feature = "sql"))]
pub use self::mock_database::MockDatabase;
#[cfg(feature = "sql")]
pub use self::sqlite_database::SqliteDatabase;

#[cfg(test)]
mod test_sqlite_db;
