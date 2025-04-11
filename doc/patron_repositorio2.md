# Refactorización completa con mejoras aplicadas

Voy a implementar todas las sugerencias para crear una estructura más robusta y mantenible. Aquí está la organización final:

```
src/
├── main.rs            # Punto de entrada
├── config.rs          # Configuración de la aplicación
├── models.rs          # Entidades de dominio
├── error.rs           # Manejo de errores
├── repositories/
│   ├── mod.rs         # Traits e implementaciones base
│   ├── mysql/
│   │   ├── mod.rs     # Configuración MySQL
│   │   └── usuario.rs # Repositorio concreto
│   └── mock/          # Implementaciones para testing
├── api/
│   ├── mod.rs         # Configuración de rutas
│   └── usuarios.rs    # Handlers de usuarios
└── app_state.rs       # Estado compartido
```

## 1. config.rs

```rust
use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub server_address: String,
    pub log_level: String,
}

impl Config {
    pub fn from_env() -> Result<Self, env::VarError> {
        Ok(Self {
            database_url: env::var("DATABASE_URL")?,
            server_address: env::var("SERVER_ADDRESS").unwrap_or_else(|_| "0.0.0.0:8080".into()),
            log_level: env::var("LOG_LEVEL").unwrap_or_else(|_| "info".into()),
        })
    }
}
```

## 2. error.rs

```rust
use actix_web::{HttpResponse, ResponseError};
use mysql::Error as MysqlError;
use serde_json::Error as JsonError;
use std::fmt;
use tracing::error;

#[derive(Debug)]
pub enum AppError {
    Database(MysqlError),
    Config(String),
    NotFound,
    Validation(String),
    Internal(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Database(e) => write!(f, "Database error: {}", e),
            Self::Config(e) => write!(f, "Configuration error: {}", e),
            Self::NotFound => write!(f, "Resource not found"),
            Self::Validation(e) => write!(f, "Validation error: {}", e),
            Self::Internal(e) => write!(f, "Internal error: {}", e),
        }
    }
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        error!("{}", self);
        
        match self {
            Self::Database(_) => HttpResponse::InternalServerError().json(
                serde_json::json!({"error": "Database operation failed"})
            ),
            Self::Config(_) => HttpResponse::InternalServerError().json(
                serde_json::json!({"error": "Configuration error"})
            ),
            Self::NotFound => HttpResponse::NotFound().json(
                serde_json::json!({"error": "Not found"})
            ),
            Self::Validation(msg) => HttpResponse::BadRequest().json(
                serde_json::json!({"error": msg})
            ),
            Self::Internal(_) => HttpResponse::InternalServerError().json(
                serde_json::json!({"error": "Internal server error"})
            ),
        }
    }
}

impl From<MysqlError> for AppError {
    fn from(value: MysqlError) -> Self {
        Self::Database(value)
    }
}

impl From<env::VarError> for AppError {
    fn from(value: env::VarError) -> Self {
        Self::Config(value.to_string())
    }
}

impl From<JsonError> for AppError {
    fn from(value: JsonError) -> Self {
        Self::Validation(value.to_string())
    }
}
```

## 3. repositories/mod.rs

```rust
mod mysql;
#[cfg(test)]
mod mock;

pub use mysql::MysqlRepository;
#[cfg(test)]
pub use mock::MockRepository;

use async_trait::async_trait;
use crate::{models::Usuario, error::AppError};

#[async_trait]
pub trait UsuarioRepository: Send + Sync {
    async fn get_by_id(&self, id: u32) -> Result<Option<Usuario>, AppError>;
    async fn get_all(&self) -> Result<Vec<Usuario>, AppError>;
    async fn create(&self, usuario: &Usuario) -> Result<(), AppError>;
    async fn update(&self, usuario: &Usuario) -> Result<(), AppError>;
    async fn delete(&self, id: u32) -> Result<bool, AppError>;
}
```

## 4. repositories/mysql/mod.rs

```rust
use mysql_async::Pool;
use crate::{error::AppError, models::Usuario};

pub struct MysqlRepository {
    pool: Pool,
}

impl MysqlRepository {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl super::UsuarioRepository for MysqlRepository {
    async fn get_by_id(&self, id: u32) -> Result<Option<Usuario>, AppError> {
        let mut conn = self.pool.get_conn().await?;
        let query = "SELECT * FROM usuarios WHERE id = ?";
        
        let usuario: Option<Usuario> = mysql_async::query(query)
            .bind(id)
            .first_async(&mut conn)
            .await?;
            
        Ok(usuario)
    }

    // Implementar otros métodos...
}
```

## 5. app_state.rs

```rust
use std::sync::Arc;
use crate::repositories::UsuarioRepository;

pub struct AppState<R: UsuarioRepository> {
    pub usuario_repository: Arc<R>,
}

impl<R: UsuarioRepository> AppState<R> {
    pub fn new(repository: R) -> Self {
        Self {
            usuario_repository: Arc::new(repository),
        }
    }
}
```

## 6. api/usuarios.rs

```rust
use actix_web::{web, HttpResponse};
use crate::{models::Usuario, app_state::AppState, error::AppError};
use super::super::repositories::UsuarioRepository;

pub async fn get_usuario<R>(
    id: web::Path<u32>,
    data: web::Data<AppState<R>>,
) -> Result<HttpResponse, AppError> 
where
    R: UsuarioRepository + 'static,
{
    let usuario = data.usuario_repository.get_by_id(id.into_inner()).await?;
    
    match usuario {
        Some(u) => Ok(HttpResponse::Ok().json(u)),
        None => Err(AppError::NotFound),
    }
}

// Implementar otros handlers...
```

## 7. main.rs

```rust
use actix_web::{web, App, HttpServer};
use tracing::info;
use tracing_subscriber::{fmt, EnvFilter};

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
    let pool = mysql_async::Pool::new(config.database_url);
    
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
```

## Mejoras clave implementadas:

1. **Configuración centralizada**:
   - Archivo `config.rs` para manejar todas las variables de entorno
   - Tipos seguros y valores por defecto

2. **Manejo mejorado de errores**:
   - Tipo `AppError` más completo
   - Implementación automática de conversiones
   - Logging integrado

3. **Soporte async completo**:
   - Uso de `mysql_async` en lugar de `mysql`
   - Trait del repositorio con `async_trait`

4. **Estado de la aplicación tipado**:
   - `AppState` genérico sobre el tipo de repositorio
   - Fácil de mockear para tests

5. **Logging con tracing**:
   - Integración con actix-web
   - Configuración por niveles

6. **Mejor organización de módulos**:
   - Separación clara de responsabilidades
   - Estructura escalable

## Para usar este código:

1. Agrega estas dependencias a tu `Cargo.toml`:

```toml
[dependencies]
actix-web = "4"
mysql_async = "0.32"
tracing = "0.1"
tracing-subscriber = "0.3"
async-trait = "0.1"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
dotenv = "0.15"
```

2. Crea un archivo `.env`:

```env
DATABASE_URL=mysql://user:password@localhost:3306/dbname
SERVER_ADDRESS=0.0.0.0:8080
LOG_LEVEL=info
```

Esta estructura proporciona una base sólida para una API REST en Rust que:
- Es fácil de mantener
- Es testeable
- Es escalable
- Tiene logging adecuado
- Maneja errores correctamente
- Sigue mejores prácticas de diseño
