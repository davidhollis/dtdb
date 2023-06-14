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
        let connection_manager = ConnectionManager::<PgConnection>::new(config.database_url.clone());

        // TODO: make some of the connection pool options (e.g., max pool size, expiration, etc.)
        //       configurable through `config`
        let connection_pool =
            Pool::builder()
                .build(connection_manager)?;
        
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