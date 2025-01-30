use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct MyObj {
    name: String,
    age: u8,
}

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}

async fn get_obj() -> impl Responder {
    let obj = MyObj {
        name: "John Doe".to_string(),
        age: 30,
    };
    HttpResponse::Ok().json(obj)
}

mod app;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    app::run().await
}
