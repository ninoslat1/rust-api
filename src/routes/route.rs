use actix_web::web::{self, get};
use crate::{controllers, middlewares::middleware::cookie_checkers};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/", web::post().to(controllers::index::login))
            .service(
                web::resource("/dashboard")
                    .wrap_fn(cookie_checkers) // Use the correct function reference
                    .route(get().to(controllers::index::get_statistic)),
            ),
    );
}