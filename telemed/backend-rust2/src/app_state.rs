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