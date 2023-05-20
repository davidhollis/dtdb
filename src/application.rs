use std::error::Error;

use crate::{view::Views, config::Config};

pub struct Application {
    pub config: Config,
    pub views: Views,
}

impl Application {
    pub fn from_config(config: Config) -> Result<Application, Box<dyn Error>> {
        Ok(Application {
            config,
            views: Views::init()?,
        })
    }
}