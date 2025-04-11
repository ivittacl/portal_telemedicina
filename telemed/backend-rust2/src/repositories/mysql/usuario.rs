use mysql_async::{prelude::*, Pool, Params};
use crate::{models::Usuario, error::AppError};
use super::super::UsuarioRepository;

#[derive(Clone)]
pub struct MysqlUsuarioRepository {
    pool: Pool,
}

impl MysqlUsuarioRepository {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl UsuarioRepository for MysqlUsuarioRepository {
    async fn get_by_id(&self, id: u32) -> Result<Option<Usuario>, AppError> {
        let mut conn = self.pool.get_conn().await?;
        let query = "SELECT id, rut, nombre, ap_paterno, ap_materno, email, telefonos, cod_zona, nivel_acceso, cod_cliente, clave_acceso, estatus FROM usuarios WHERE id = ?";
        
        let usuario = conn.exec_first::<Usuario, _, _>(query, (id,))
            .await?;
            
        Ok(usuario)
    }

    async fn get_all(&self) -> Result<Vec<Usuario>, AppError> {
        let mut conn = self.pool.get_conn().await?;
        let query = "SELECT id, rut, nombre, ap_paterno, ap_materno, email, telefonos, cod_zona, nivel_acceso, cod_cliente, clave_acceso, estatus FROM usuarios";
        
        let usuarios = conn.query::<Usuario, _>(query)
            .await?;
            
        Ok(usuarios)
    }

    async fn create(&self, usuario: &Usuario) -> Result<(), AppError> {
        let mut conn = self.pool.get_conn().await?;
        let query = r"
            INSERT INTO usuarios 
            (rut, nombre, ap_paterno, ap_materno, email, telefonos, cod_zona, nivel_acceso, cod_cliente, clave_acceso, estatus) 
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)";
        
        conn.exec_drop(query, (
            &usuario.rut,
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
        )).await?;
        
        Ok(())
    }

    async fn update(&self, usuario: &Usuario) -> Result<(), AppError> {
        let mut conn = self.pool.get_conn().await?;
        let query = r"
            UPDATE usuarios SET 
            rut = ?, nombre = ?, ap_paterno = ?, ap_materno = ?, email = ?, telefonos = ?, 
            cod_zona = ?, nivel_acceso = ?, cod_cliente = ?, clave_acceso = ?, estatus = ?
            WHERE id = ?";
        
        conn.exec_drop(query, (
            &usuario.rut,
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
            &usuario.id,
        )).await?;
        
        Ok(())
    }

    async fn delete(&self, id: u32) -> Result<bool, AppError> {
        let mut conn = self.pool.get_conn().await?;
        let query = "DELETE FROM usuarios WHERE id = ?";
        
        let result = conn.exec_drop(query, (id,))
            .await?;
            
        Ok(result.affected_rows() > 0)
    }
}