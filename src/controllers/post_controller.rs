// src/controllers/post_controller.rs

use actix_web::{web, HttpResponse, Responder};
use crate::models::post_model::{CreatePostRequest, PathInfo};

pub async fn create_post(post: web::Json<CreatePostRequest>) -> impl Responder {
    HttpResponse::Ok().json(format!("Post created: {}", post.title))
}

pub async fn get_post_by_id(path: web::Path<PathInfo>) -> impl Responder {
    let post_id = path.postid;
    HttpResponse::Ok().json(format!("Fetching post with ID: {}", post_id))
}
