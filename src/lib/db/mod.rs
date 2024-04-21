pub mod model;

use sqlx;

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

    pub fn get_pool(&self) -> &sqlx::Pool<sqlx::Postgres> {
        return &self.0;
    }
}
