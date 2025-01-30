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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/obj", web::get().to(get_obj))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
