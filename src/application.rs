use std::time::Duration;

use diesel::pg::PgConnection;
use diesel::r2d2::{Pool, ConnectionManager};

use crate::{view::Views, config::Config};

pub struct Application {
    pub config: Config,
    pub views: Views,
    db_connection_pool: Pool<ConnectionManager<PgConnection>>,
}

impl Application {
    pub fn from_config(config: Config) -> anyhow::Result<Application> {
        let connection_manager = ConnectionManager::<PgConnection>::new(config.database.url.clone());

        let mut connection_pool_builder =
            Pool::builder()
                .min_idle(config.database.connection_pool.min_idle_count)
                .max_lifetime(config.database.connection_pool.max_lifetime_ms.map(Duration::from_millis))
                .idle_timeout(config.database.connection_pool.idle_timeout_ms.map(Duration::from_millis))
                .test_on_check_out(config.database.connection_pool.test_connection_on_checkout);
        
        if let Some(max_size) = config.database.connection_pool.max_size {
            connection_pool_builder = connection_pool_builder.max_size(max_size);
        }

        if let Some(connection_timeout_ms) = config.database.connection_pool.connection_timeout_ms {
            connection_pool_builder = connection_pool_builder.connection_timeout(Duration::from_millis(connection_timeout_ms));
        }

        let connection_pool = connection_pool_builder.build(connection_manager)?;
        
        Ok(Application {
            config,
            views: Views::init()?,
            db_connection_pool: connection_pool,
        })
    }

    pub fn with_db_connection<F, T>(&self, body: F) -> anyhow::Result<T>
    where
        F: FnOnce(&mut PgConnection) -> anyhow::Result<T>
    {
        let conn = &mut self.db_connection_pool.get()?;
        body(conn)
    }
}