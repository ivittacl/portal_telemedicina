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