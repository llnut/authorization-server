use config::ConfigError;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::MysqlConnection;
//use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
//use tracing::info;

pub type DbPool = Pool<ConnectionManager<MysqlConnection>>;

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub listen_addr: SocketAddr,
    pub database_url: String,
}

impl Config {
    pub fn try_from_env() -> Result<Self, ConfigError> {
        //dotenv().ok();
        let mut cfg = config::Config::new();
        cfg.merge(config::Environment::new())?;
        cfg.try_into()
    }

    pub async fn build_db_pool(&self) -> DbPool {
        let manager = ConnectionManager::<MysqlConnection>::new(&self.database_url);
        Pool::builder()
            .max_size(10)
            .build(manager)
            .expect("failed to create db pool")
    }
}
