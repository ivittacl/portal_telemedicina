use std::sync::Arc;
use crate::repositories::UsuarioRepository;

#[derive(Clone)]
pub struct AppState<R: UsuarioRepository> {
    pub usuario_repo: Arc<R>,
}

impl<R: UsuarioRepository> AppState<R> {
    pub fn new(repository: R) -> Self {
        Self {
            usuario_repo: Arc::new(repository),
        }
    }
}