// src/routes/user_routes.rs

use actix_web::web;
use crate::controllers::user_controller;

pub fn user_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/signup").route(web::post().to(user_controller::signup))
    );
    cfg.service(
        web::resource("/login").route(web::post().to(user_controller::login))
    );
}
