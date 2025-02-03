use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

// Define a struct for the response
#[derive(Serialize)]
struct MyResponse {
    message: String,
}

// Define a handler function
async fn index() -> impl Responder {
    HttpResponse::Ok().json(MyResponse {
        message: "Hello, world!".to_string(),
    })
}

// Define another handler function
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/echo", web::post().to(echo))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
