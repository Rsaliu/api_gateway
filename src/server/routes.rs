use actix_web::{web};
use crate::server::handlers;


pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            .route("/signup", web::post().to(handlers::signup))
    );
}

