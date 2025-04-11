use mysql_async::{prelude::*, Pool, Params, Row};
use crate::{models::Usuario, error::AppError};
use crate::repositories::UsuarioRepository;

#[derive(Clone)]
pub struct MysqlRepository {
    pool: Pool,
}

impl MysqlRepository {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }

    async fn query_single(&self, query: &str, params: Params) -> Result<Option<Usuario>, AppError> {
        let mut conn = self.pool.get_conn().await?;
        let row: Option<Row> = conn.exec_first(query, params).await?;
        
        Ok(row.map(|r| Usuario::from_row(r)))
    }

    async fn query_multiple(&self, query: &str, params: Params) -> Result<Vec<Usuario>, AppError> {
        let mut conn = self.pool.get_conn().await?;
        let rows: Vec<Row> = conn.exec(query, params).await?;
        
        Ok(rows.into_iter().map(Usuario::from_row).collect())
    }
}

#[async_trait::async_trait]
impl UsuarioRepository for MysqlRepository {
    async fn get_by_id(&self, id: u32) -> Result<Option<Usuario>, AppError> {
        let query = "SELECT id, rut, nombre, ap_paterno, ap_materno, email, telefonos, cod_zona, nivel_acceso, cod_cliente, clave_acceso, estatus FROM usuarios WHERE id = ?";
        self.query_single(query, Params::Positional(vec![id.into()])).await
    }

    async fn get_all(&self) -> Result<Vec<Usuario>, AppError> {
        let query = "SELECT id, rut, nombre, ap_paterno, ap_materno, email, telefonos, cod_zona, nivel_acceso, cod_cliente, clave_acceso, estatus FROM usuarios";
        self.query_multiple(query, Params::Empty).await
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
        
        let result = conn.exec_iter(query, (id,)).await?;
        Ok(result.affected_rows() > 0)
    }
}