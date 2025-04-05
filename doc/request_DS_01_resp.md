# Backend Rust para Telemedicina

Voy a crear un backend en Rust que cumpla con todos los requisitos que has especificado. El sistema incluirá conexión a MySQL, API HTTP con las rutas solicitadas, manejo de errores profesional y todas las características de seguridad y robustez que mencionas.

## Estructura del Proyecto

Primero, creemos el proyecto con Cargo:

```bash
cargo new backend_rust
cd backend_rust
```

## Cargo.toml

```toml
[package]
name = "backend_rust"
version = "0.1.0"
edition = "2021"

[dependencies]
actix-web = "4"
actix-rt = "2"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
mysql = "24.0"
dotenv = "0.15"
thiserror = "1.0"
env_logger = "0.10"
log = "0.4"
sha1 = "0.10"
futures = "0.3"
lazy_static = "1.4"
```

## Código Principal (main.rs)

```rust
use actix_web::{web, App, HttpResponse, HttpServer, Responder, delete, get, post};
use mysql::prelude::*;
use mysql::*;
use serde::{Deserialize, Serialize};
use std::env;
use dotenv::dotenv;
use thiserror::Error;
use log::{info, error};
use lazy_static::lazy_static;
use std::sync::Mutex;

// Configuración de la aplicación
#[derive(Debug, Clone)]
struct AppConfig {
    db_url: String,
}

// Estructuras de datos
#[derive(Debug, Serialize, Deserialize)]
struct Usuario {
    id: u32,
    rut: String,
    nombre: String,
    ap_paterno: String,
    ap_materno: String,
    email: String,
    telefonos: String,
    cod_zona: String,
    nivel_acceso: u8,
    cod_cliente: String,
    clave_acceso: String,
    estatus: u8,
}

#[derive(Debug, Serialize, Deserialize)]
struct UsuarioInput {
    nombre: String,
    ap_paterno: String,
    ap_materno: String,
    email: String,
    telefonos: String,
    cod_zona: String,
    nivel_acceso: u8,
    cod_cliente: String,
    clave_acceso: String,
    estatus: u8,
}

#[derive(Debug, Serialize, Deserialize)]
struct UsuarioResponse {
    datos: Option<Usuario>,
    cod_err: u8,
    msg_err: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct UsuariosListResponse {
    datos: Vec<Usuario>,
    cod_err: u8,
    msg_err: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct BasicResponse {
    cod_err: u8,
    msg_err: String,
}

// Errores de la aplicación
#[derive(Error, Debug)]
enum AppError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] mysql::Error),
    
    #[error("Environment error: {0}")]
    EnvError(#[from] env::VarError),
    
    #[error("JSON parsing error: {0}")]
    JsonError(#[from] serde_json::Error),
    
    #[error("Not found")]
    NotFound,
    
    #[error("Internal server error")]
    InternalError,
}

impl actix_web::error::ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        error!("Error occurred: {}", self);
        match self {
            AppError::DatabaseError(_) => HttpResponse::InternalServerError().json(BasicResponse {
                cod_err: 1,
                msg_err: "Database error".to_string(),
            }),
            AppError::EnvError(_) => HttpResponse::InternalServerError().json(BasicResponse {
                cod_err: 1,
                msg_err: "Configuration error".to_string(),
            }),
            AppError::JsonError(_) => HttpResponse::BadRequest().json(BasicResponse {
                cod_err: 1,
                msg_err: "Invalid JSON".to_string(),
            }),
            AppError::NotFound => HttpResponse::NotFound().json(BasicResponse {
                cod_err: 1,
                msg_err: "Not found".to_string(),
            }),
            AppError::InternalError => HttpResponse::InternalServerError().json(BasicResponse {
                cod_err: 1,
                msg_err: "Internal server error".to_string(),
            }),
        }
    }
}

// Pool de conexiones compartido
lazy_static! {
    static ref POOL: Mutex<Option<Pool>> = Mutex::new(None);
}

// Inicializar el pool de conexiones
fn init_db_pool(db_url: &str) -> Result<(), AppError> {
    let pool = Pool::new(db_url)?;
    *POOL.lock().unwrap() = Some(pool);
    Ok(())
}

// Obtener conexión del pool
fn get_conn() -> Result<PooledConn, AppError> {
    POOL.lock()
        .unwrap()
        .as_ref()
        .ok_or(AppError::InternalError)?
        .get_conn()
        .map_err(AppError::DatabaseError)
}

// Handlers de la API
#[get("/usuarios/{id}")]
async fn get_usuario(id: web::Path<u32>) -> Result<HttpResponse, AppError> {
    let mut conn = get_conn()?;
    let query = "SELECT id, rut, nombre, ap_paterno, ap_materno, email, telefonos, cod_zona, nivel_acceso, cod_cliente, clave_acceso, estatus FROM usuarios WHERE id = ?";
    
    let usuario: Option<Usuario> = conn.exec_first(
        query,
        (id.into_inner(),),
    )?;
    
    match usuario {
        Some(u) => Ok(HttpResponse::Ok().json(UsuarioResponse {
            datos: Some(u),
            cod_err: 0,
            msg_err: "OK".to_string(),
        })),
        None => Ok(HttpResponse::Ok().json(UsuarioResponse {
            datos: None,
            cod_err: 1,
            msg_err: "Usuario no encontrado".to_string(),
        })),
    }
}

#[get("/usuarios")]
async fn get_usuarios() -> Result<HttpResponse, AppError> {
    let mut conn = get_conn()?;
    let query = "SELECT id, rut, nombre, ap_paterno, ap_materno, email, telefonos, cod_zona, nivel_acceso, cod_cliente, clave_acceso, estatus FROM usuarios ORDER BY id";
    
    let usuarios: Vec<Usuario> = conn.query_map(query, |(id, rut, nombre, ap_paterno, ap_materno, email, telefonos, cod_zona, nivel_acceso, cod_cliente, clave_acceso, estatus)| {
        Usuario {
            id,
            rut,
            nombre,
            ap_paterno,
            ap_materno,
            email,
            telefonos,
            cod_zona,
            nivel_acceso,
            cod_cliente,
            clave_acceso,
            estatus,
        }
    })?;
    
    Ok(HttpResponse::Ok().json(UsuariosListResponse {
        datos: usuarios,
        cod_err: 0,
        msg_err: "OK".to_string(),
    }))
}

#[post("/usuarios")]
async fn post_usuario(usuario: web::Json<UsuarioInput>) -> Result<HttpResponse, AppError> {
    let mut conn = get_conn()?;
    
    // Verificar si el usuario existe
    let exists: Option<u32> = conn.exec_first(
        "SELECT id FROM usuarios WHERE email = ?",
        (&usuario.email,),
    )?;
    
    if let Some(id) = exists {
        // Actualizar usuario existente
        conn.exec_drop(
            "UPDATE usuarios SET nombre = ?, ap_paterno = ?, ap_materno = ?, telefonos = ?, cod_zona = ?, nivel_acceso = ?, cod_cliente = ?, clave_acceso = ?, estatus = ? WHERE id = ?",
            (
                &usuario.nombre,
                &usuario.ap_paterno,
                &usuario.ap_materno,
                &usuario.telefonos,
                &usuario.cod_zona,
                &usuario.nivel_acceso,
                &usuario.cod_cliente,
                &usuario.clave_acceso,
                &usuario.estatus,
                id,
            ),
        )?;
    } else {
        // Insertar nuevo usuario
        conn.exec_drop(
            "INSERT INTO usuarios (nombre, ap_paterno, ap_materno, email, telefonos, cod_zona, nivel_acceso, cod_cliente, clave_acceso, estatus) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
            (
                &usuario.nombre,
                &usuario.ap_paterno,
                &usuario.ap_materno,
                &usuario.email,
                &usuario.telefonos,
                &usuario.cod_zona,
                &usuario.nivel_acceso,
                &usuario.cod_cliente,
                &usuario.clave_acceso,
                &usuario.estatus,
            ),
        )?;
    }
    
    Ok(HttpResponse::Ok().json(BasicResponse {
        cod_err: 0,
        msg_err: "OK".to_string(),
    }))
}

#[delete("/usuarios/{id}")]
async fn delete_usuario(id: web::Path<u32>) -> Result<HttpResponse, AppError> {
    let mut conn = get_conn()?;
    
    let affected = conn.exec_drop(
        "DELETE FROM usuarios WHERE id = ?",
        (id.into_inner(),),
    )?;
    
    if affected.rows_affected() > 0 {
        Ok(HttpResponse::Ok().json(BasicResponse {
            cod_err: 0,
            msg_err: "OK".to_string(),
        }))
    } else {
        Ok(HttpResponse::Ok().json(BasicResponse {
            cod_err: 1,
            msg_err: "Usuario no encontrado".to_string(),
        }))
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Configuración inicial
    dotenv().ok();
    env_logger::init();
    
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    
    // Inicializar pool de conexiones
    init_db_pool(&db_url).expect("Failed to create database pool");
    
    info!("Starting server...");
    
    // Iniciar servidor HTTP
    HttpServer::new(|| {
        App::new()
            .service(get_usuario)
            .service(get_usuarios)
            .service(post_usuario)
            .service(delete_usuario)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
```

## Archivo .env

```env
DATABASE_URL=mysql://user:password@localhost:3306/telemedicina
RUST_LOG=info
```

## Explicación del Código

1. **Configuración y Dependencias**:
   - Usamos `actix-web` como framework web
   - `mysql` para la conexión a la base de datos
   - `dotenv` para manejar variables de entorno
   - `thiserror` para manejo profesional de errores
   - `env_logger` para logging

2. **Pool de Conexiones**:
   - Usamos `lazy_static` para crear un pool de conexiones compartido
   - El pool se inicializa una vez al inicio de la aplicación

3. **Estructuras de Datos**:
   - Definimos estructuras para los usuarios y las respuestas JSON
   - Implementamos `Serialize` y `Deserialize` para conversión JSON

4. **Manejo de Errores**:
   - Definimos un enum `AppError` con todos los posibles errores
   - Implementamos `ResponseError` para convertir errores en respuestas HTTP apropiadas

5. **Endpoints**:
   - Implementamos los 4 endpoints solicitados con sus respectivas consultas SQL
   - Usamos consultas preparadas para prevenir SQL injection
   - Cada endpoint devuelve el formato JSON especificado

6. **Seguridad**:
   - Las credenciales de la BD se cargan desde variables de entorno
   - Todas las consultas SQL usan parámetros preparados
   - Logging de todos los errores

## Ejecución del Proyecto

1. Crea un archivo `.env` con tus credenciales de MySQL
2. Ejecuta el servidor:

```bash
cargo run
```

El servidor estará disponible en `http://localhost:8080` con los siguientes endpoints:

- `GET /usuarios/{id}` - Obtener un usuario por ID
- `GET /usuarios` - Listar todos los usuarios
- `POST /usuarios` - Crear o actualizar un usuario
- `DELETE /usuarios/{id}` - Eliminar un usuario

Este código cumple con todos los requisitos funcionales y no funcionales que especificaste, incluyendo manejo profesional de errores, seguridad, pool de conexiones y logging.
