use actix_web::web;

use crate::controllers;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api") // Define the API scope
            .route("/", web::get().to(controllers::index::index)), // Route to the index controller
    );
}