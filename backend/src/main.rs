use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use actix_cors::Cors;
use actix_web::{App, HttpServer, web};
use anyhow::Context;

use crate::{
    api::{
        plugins::{list_plugins, plugin_metadata},
        ws::ws,
    },
    config::Config,
    plugins::Plugin,
    state::AppState,
};

mod api;
mod config;
mod plugins;
mod state;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();

    let config = Config::from_env().with_context(|| "Failed to load configuration")?;
    unsafe {
        std::env::set_var("RUST_LOG", &config.log_level);
    }
    env_logger::init();

    log::info!("Starting backend server with config: {:?}", config);
    log::info!("Loading plugins from directory: {}", config.plugins_dir);

    let plugins = Arc::new(Mutex::new(
        Plugin::load_from_dir(&config.plugins_dir)?
            .into_iter()
            .filter_map(|plugin| match plugin {
                Ok(plugin) => {
                    log::info!(
                        "Successfully loaded plugin: {} ({} {}.{}.{})",
                        plugin.metadata.display_name,
                        plugin.metadata.name,
                        plugin.metadata.version.major,
                        plugin.metadata.version.minor,
                        plugin.metadata.version.patch,
                    );
                    Some(plugin)
                }
                Err(err) => {
                    log::warn!("Failed to load plugin: {:?}", err);
                    None
                }
            })
            .collect::<Vec<_>>(),
    ));
    log::info!("Loaded {} plugins total", plugins.lock().unwrap().len());

    let server_addr = config.addr();
    log::info!(
        "Starting HTTP server on {}:{}",
        server_addr.0,
        server_addr.1
    );

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState {
                plugins: plugins.clone(),
                sessions: Arc::new(Mutex::new(HashMap::new())),
            }))
            .wrap(Cors::permissive())
            .service(ws)
            .service(plugin_metadata)
            .service(list_plugins)
    })
    .bind(&server_addr)?
    .run()
    .await?;

    log::info!("Server stopped");
    Ok(())
}
