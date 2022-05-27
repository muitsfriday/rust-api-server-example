#![allow(dead_code)]
use actix_web::{web, Responder, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct RequestBody {
    title: String,
    description: String,
    content: String,
    tags: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct Response {
    id: String,
    post: RequestBody,
}

pub async fn handle(req_body: web::Json<RequestBody>) -> Result<impl Responder> {
    let result = Response {
        id: String::from("mock"),
        post: req_body.into_inner(),
    };

    Ok(web::Json(result))
}