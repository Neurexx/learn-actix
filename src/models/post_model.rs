// src/models/post_model.rs

use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreatePostRequest {
    pub title: String,
    pub content: String,
}

#[derive(Deserialize)]
pub struct PathInfo {
    pub postid: u32,
}
