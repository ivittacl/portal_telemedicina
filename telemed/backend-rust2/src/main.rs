use actix_web::{web, App, HttpServer};
use tracing::info;
use tracing_subscriber::{fmt, EnvFilter};
use mysql_async::Opts;

mod config;
mod models;
mod error;
mod repositories;
mod api;
mod app_state;

use crate::{
    config::Config,
    repositories::{UsuarioRepository, MysqlRepository},
    app_state::AppState,
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Configuración
    let config = Config::from_env().expect("Failed to load configuration");
    
    // Logging
    fmt()
        .with_env_filter(EnvFilter::new(config.log_level))
        .init();
    
    // Pool de conexiones MySQL
    let opts = Opts::from_url(&config.database_url).expect("Invalid database URL");
    let pool = mysql_async::Pool::new(opts);
    
    // Repositorio
    let repository = MysqlRepository::new(pool);
    let app_state = AppState::new(repository);
    
    info!("Starting server on {}", config.server_address);
    
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .configure(api::configure)
    })
    .bind(config.server_address)?
    .run()
    .await
}