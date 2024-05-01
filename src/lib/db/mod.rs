pub mod models;
pub mod query;

use dotenvy;
use sqlx;

pub type DatabaseDialect = sqlx::Postgres;
pub type DatabasePool = sqlx::Pool<sqlx::Postgres>;

pub struct Database<D: sqlx::Database>(sqlx::Pool<D>);

impl Database<sqlx::Postgres> {
    pub async fn new(conn_str: &str) -> Self {
        let pool = sqlx::postgres::PgPoolOptions::new().connect(conn_str).await;

        match pool {
            Ok(pool) => Self(pool),
            Err(err) => {
                panic!("Database connection error {}", err);
            }
        }
    }

    pub async fn default() -> Self {
        return match Self::get_default_connection_string() {
            Ok(conn_str) => Self::new(&conn_str).await,
            _ => panic!("Invalid connection string to create a connection."),
        };
    }

    pub fn get_pool(&self) -> &sqlx::Pool<sqlx::Postgres> {
        return &self.0;
    }

    pub fn get_default_connection_string() -> Result<String, dotenvy::Error> {
        let conn_string = dotenvy::var("DATABASE_URL")?;
        return Ok(conn_string);
    }
}
