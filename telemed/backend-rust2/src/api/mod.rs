use actix_web::web;
use crate::repositories::MysqlRepository;

mod usuarios;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(
                web::resource("/usuarios")
                    .route(web::get().to(usuarios::get_all::<MysqlRepository>))
                    .route(web::post().to(usuarios::create::<MysqlRepository>))
            )
            .service(
                web::resource("/usuarios/{id}")
                    .route(web::get().to(usuarios::get_by_id::<MysqlRepository>))
                    .route(web::put().to(usuarios::update::<MysqlRepository>))
                    .route(web::delete().to(usuarios::delete::<MysqlRepository>))
            )
    );
}