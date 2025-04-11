pub mod mysql;

#[cfg(test)]
pub mod mock;

pub use mysql::MysqlRepository;
#[cfg(test)]
pub use mock::MockRepository;

use async_trait::async_trait;
use crate::{models::Usuario, error::AppError};

#[async_trait]
pub trait UsuarioRepository: Send + Sync + Clone {
    async fn get_by_id(&self, id: u32) -> Result<Option<Usuario>, AppError>;
    async fn get_all(&self) -> Result<Vec<Usuario>, AppError>;
    async fn create(&self, usuario: &Usuario) -> Result<(), AppError>;
    async fn update(&self, usuario: &Usuario) -> Result<(), AppError>;
    async fn delete(&self, id: u32) -> Result<bool, AppError>;
}