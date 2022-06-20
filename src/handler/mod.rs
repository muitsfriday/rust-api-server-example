use actix_web::{HttpResponse, Responder};

pub mod create_post;
pub mod get_post;

pub async fn test_handler() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}