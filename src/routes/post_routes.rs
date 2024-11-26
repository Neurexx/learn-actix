// src/routes/post_routes.rs

use actix_web::web;
use crate::controllers::post_controller;

pub fn post_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/create").route(web::post().to(post_controller::create_post))
    );
    cfg.service(
        web::resource("/{postid}").route(web::get().to(post_controller::get_post_by_id))
    );
}
