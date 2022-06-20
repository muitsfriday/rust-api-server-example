#![allow(dead_code)]
use actix_web::{web, Responder, Result};
use serde::Serialize;

use crate::service::Post;

#[derive(Debug, Serialize)]
pub struct Response {
    post: Post,
}

pub async fn handle(id: web::Path<String>) -> Result<impl Responder> {
    let mut post = Post::default();
    post.id = id.into_inner();

    let result = Response { post };

    Ok(web::Json(result))
}