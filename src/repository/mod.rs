use sqlx::migrate::MigrateDatabase;
use sqlx::{Error, Executor, Pool, Row, Sqlite};
use sqlx::sqlite::SqlitePoolOptions;

pub struct DatabaseManager {}

const DATABASE_URL: &'static str = "sqlite:./sandbox.data";

impl DatabaseManager {
    pub async fn initialize_database(pool: &Pool<Sqlite>) -> Result<(), Error> {
        pool.execute(include_str!("../../db_init.sql")).await.map(|_| ())
    }

    pub async fn get_connection() -> Result<Pool<Sqlite>, Error> {
         SqlitePoolOptions::new().max_connections(1).connect(DATABASE_URL).await
    }

    pub async fn is_database_created(should_create: bool) -> Result<bool, Error> {
        // Check if the database exists, if not it is created
        if !Sqlite::database_exists(DATABASE_URL).await? {
            if !should_create { return Ok(false); }
            Sqlite::create_database(DATABASE_URL).await.map(|_| true)?;
        }
        Ok(true)
    }

    pub async fn is_database_initialized(pool: &Pool<Sqlite>) -> Result<bool, Error> {
        //TODO: Create a more sophisticated way to check if the database is already initilized

        // If the database already exists, check if the tables are there
        let result = sqlx::query("SELECT name FROM sqlite_schema WHERE type ='table' AND name NOT LIKE 'sqlite_%';",)
            .fetch_all(pool).await.unwrap();

        for (idx, row) in result.iter().enumerate() {
            let table_name = row.get::<String, &str>("name");
            if table_name == "tb_user" {
                return Ok(true);
            }
        }
        Ok(false)
    }

    //TODO: Create the unit tests for the database management system
}