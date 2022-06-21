#![allow(dead_code)]
use actix_web::{error, web, Responder, Result};
use serde::{Deserialize, Serialize};

use crate::service::Post;
use crate::AppData;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RequestBody {
    title: String,
    description: String,
    content: String,
    tags: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct Response {
    post: Post,
}

impl Into<Post> for RequestBody {
    fn into(self) -> Post {
        Post {
            id: String::default(),
            title: self.title,
            description: self.description,
            content: self.content,
            tags: self.tags,
            created_at: None,
            updated_at: None,
            deleted_at: None,
        }
    }
}

pub async fn handle(
    req_body: web::Json<RequestBody>,
    app_data: web::Data<AppData>,
) -> Result<impl Responder> {
    let body = req_body.into_inner();
    let r = app_data.post_service.create(body.into());

    match r {
        Ok(post) => Ok(web::Json(Response { post })),
        Err(e) => Err(error::ErrorBadRequest(e.to_string())),
    }
}
