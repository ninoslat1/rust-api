use actix_web::web;

use crate::controllers;

pub fn config(cfg: &mut web::ServiceConfig) {
    
    cfg.service(
        web::scope("/api") // Define the API scope
            .route("/", web::post().to(controllers::index::login))
            .route("/dashboard", web::get().to(controllers::index::get_statistic))
    );
}