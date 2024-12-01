use actix_web::{ App, HttpServer};
use dotenv::dotenv;
use std::env;

mod controllers; 
mod routes; 
mod libs;
mod models;
mod middlewares;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let port: u16 = env::var("APP_PORT")
        .unwrap()
        .parse()
        .expect("APP_PORT must be a number");

    // Start the HTTP server
    HttpServer::new(|| {
        App::new()
            .configure(routes::route::config) 
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}