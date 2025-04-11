use actix_web::web;
use crate::repositories::mysql::MysqlUsuarioRepository;

mod usuarios;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(
                web::resource("/usuarios")
                    .route(web::get().to(usuarios::get_all))
                    .route(web::post().to(usuarios::create))
            )
            .service(
                web::resource("/usuarios/{id}")
                    .route(web::get().to(usuarios::get_by_id))
                    .route(web::put().to(usuarios::update))
                    .route(web::delete().to(usuarios::delete))
            )
    );
}