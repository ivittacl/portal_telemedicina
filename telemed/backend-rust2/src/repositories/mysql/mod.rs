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