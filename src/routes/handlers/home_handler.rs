use actix_web::{get, web, Responder};

use crate::utils::api_response::ApiResponse;

#[get("/hello/{name}")]
pub async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}

#[get("/test")]
pub async fn test() -> impl Responder {
    api_response::ApiResPonse::new(200, "Test")
}
