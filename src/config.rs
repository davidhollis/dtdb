use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(default = "default_database_url")]
    pub database_url: String,
    pub bind: Bind,
    pub log: log4rs::config::RawConfig,
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