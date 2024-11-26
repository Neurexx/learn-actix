// src/main.rs

use actix_web::{App, HttpServer, web};
use actix_cors::Cors;
use crate::routes::{user_routes,post_routes};

mod controllers;
mod routes;
mod models;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
        .wrap(
            Cors::default()
                .allowed_origin("http://localhost:5173") // Allow requests from the frontend at this origin
                .allowed_methods(vec!["GET", "POST", "DELETE"]) // Specify allowed HTTP methods
                .allowed_headers(vec![actix_web::http::header::AUTHORIZATION, actix_web::http::header::CONTENT_TYPE, actix_web::http::header::ACCESS_CONTROL_ALLOW_ORIGIN])
                .max_age(3600)
        )
            .service(
                web::scope("/users")
                    .configure(user_routes::user_routes) // Forward /users/* to user_routes
            )
            .service(
                web::scope("/posts")
                    .configure(post_routes::post_routes) // Forward /posts/* to post_routes
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
