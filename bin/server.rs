use std::{path::PathBuf, error::Error, sync::Arc, net::{SocketAddr, IpAddr}};

use axum::Server;
use clap::Parser;
use dotenvy::dotenv;
use dtdb::{config::{Config, Bind}, application::Application, routes};
use hyperlocal::UnixServerExt;

#[derive(Parser)]
#[command(name = "dtdb")]
#[command(about = "Runs the dtdb server", long_about = None)]
struct CLI {
    /// Path to the config file to load
    #[arg(short, long, value_name = "FILE")]
    config: PathBuf,
}

#[tokio::main]
async fn main() -> Result<(),Box<dyn Error>> {
    // Load environment variables from the nearest .env
    let dotenv_result = dotenv();

    // Load the config file
    let args = CLI::parse();
    let config_raw = std::fs::read_to_string(args.config)?;
    let config = json5::from_str::<Config>(&config_raw)?;

    // Initialize logging
    log4rs::config::init_raw_config(config.log.clone())?;
    match dotenv_result {
        Ok(path) => log::info!("Found and loaded env file at {:?}", path),
        Err(_) => log::info!("Found no env file. Skipping."),
    }
    log::info!("Building web server with config {:?}", config);

    let app = Arc::new(Application::from_config(config)?);
    let routes = routes::build_routes(app.clone());

    match &app.config.bind {
        Bind::Socket { address, port } => {
            let addr = SocketAddr::new(address.parse::<IpAddr>()?, *port);
            log::info!("Listening on {}", addr);
            Server::bind(&addr.into())
                .serve(routes.into_make_service())
                .await?
        },
        Bind::UnixSocket { path } => {
            log::info!("Listening on unix://{}", path);
            Server::bind_unix(path)?
                .serve(routes.into_make_service())
                .await?
        }
    };

    Ok(())
}
