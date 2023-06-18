use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct Config {
    pub database: Database,
    pub bind: Bind,
    pub log: log4rs::config::RawConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Database {
    #[serde(default = "default_database_url")]
    pub url: String,
    pub connection_pool: ConnectionPool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConnectionPool {
    pub max_size: Option<u32>,
    pub min_idle_count: Option<u32>,
    pub max_lifetime_ms: Option<u64>,
    pub idle_timeout_ms: Option<u64>,
    pub connection_timeout_ms: Option<u64>,
    #[serde(default = "default_test_on_checkout")]
    pub test_connection_on_checkout: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Bind {
    Socket {
        address: String,
        port: u16
    },
    UnixSocket {
        path: String,
    }
}

fn default_database_url() -> String {
    std::env::var("DATABASE_URL").expect("No database URL found in config or environment")
}

fn default_test_on_checkout() -> bool {
    true
}