uuse actix_web::{web, HttpResponse};
use crate::{
    models::{Usuario, UsuarioInput},
    app_state::AppState,
    error::AppError
};
use super::super::repositories::UsuarioRepository;

pub async fn get_by_id<R>(
    id: web::Path<u32>,
    data: web::Data<AppState<R>>,
) -> Result<HttpResponse, AppError>
where
    R: UsuarioRepository + 'static,
{
    let usuario = data.usuario_repo.get_by_id(id.into_inner()).await?;
    match usuario {
        Some(u) => Ok(HttpResponse::Ok().json(u)),
        None => Err(AppError::NotFound),
    }
}

pub async fn get_all<R>(
    data: web::Data<AppState<R>>,
) -> Result<HttpResponse, AppError>
where
    R: UsuarioRepository + 'static,
{
    let usuarios = data.usuario_repo.get_all().await?;
    Ok(HttpResponse::Ok().json(usuarios))
}

pub async fn create<R>(
    usuario: web::Json<UsuarioInput>,
    data: web::Data<AppState<R>>,
) -> Result<HttpResponse, AppError>
where
    R: UsuarioRepository + 'static,
{
    let usuario = usuario.into_inner().try_into()?;
    data.usuario_repo.create(&usuario).await?;
    Ok(HttpResponse::Created().finish())
}

pub async fn update<R>(
    id: web::Path<u32>,
    usuario: web::Json<UsuarioInput>,
    data: web::Data<AppState<R>>,
) -> Result<HttpResponse, AppError>
where
    R: UsuarioRepository + 'static,
{
    let mut usuario: Usuario = usuario.into_inner().try_into()?;
    usuario.id = id.into_inner();
    data.usuario_repo.update(&usuario).await?;
    Ok(HttpResponse::Ok().finish())
}

pub async fn delete<R>(
    id: web::Path<u32>,
    data: web::Data<AppState<R>>,
) -> Result<HttpResponse, AppError>
where
    R: UsuarioRepository + 'static,
{
    let deleted = data.usuario_repo.delete(id.into_inner()).await?;
    if deleted {
        Ok(HttpResponse::NoContent().finish())
    } else {
        Err(AppError::NotFound)
    }
}