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
    repositories::MysqlRepository,
    app_state::AppState,
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 1. Cargar variables de entorno
    dotenv::dotenv().expect("Failed to load .env file");
    
    // 2. Configuración
    let config = Config::from_env().expect("Failed to load configuration");
    
    // 3. Logging
    fmt()
        .with_env_filter(EnvFilter::new(config.log_level))
        .init();
    
    // 4. Pool de conexiones MySQL
    let opts = Opts::from_url(&config.database_url).expect("Invalid database URL");
    let pool = mysql_async::Pool::new(opts);
    
    // 5. Repositorio
    let repository = MysqlRepository::new(pool);
    let app_state = AppState::new(repository);
    
    info!("Starting server on {}", config.server_address);
    
    // 6. Iniciar servidor
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .configure(api::configure)
    })
    .bind(config.server_address)?
    .run()
    .await
}