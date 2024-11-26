// src/controllers/user_controller.rs

use actix_web::{web, HttpResponse, Responder};
use crate::models::user_model::{SignupRequest, LoginRequest};
use serde_json::json;

pub async fn signup(user: web::Json<SignupRequest>) -> impl Responder {
    HttpResponse::Ok().json(json!({
        "message": format!("User signed up {}", user.username)
    }))
}

pub async fn login(user: web::Json<LoginRequest>) -> impl Responder {
    HttpResponse::Ok().json(json!({"message":user.email}))
}
