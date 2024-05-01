use crate::db::DatabasePool;

pub mod clip;
pub mod datetime;
pub mod db;

#[derive(Clone)]
pub struct AppData {
    pub db_pool: DatabasePool,
}
