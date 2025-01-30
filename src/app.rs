use actix_web::{web, App, HttpServer};
mod routes;

pub async fn run() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().configure(crate::app::routes::config) // Update the path to the routes module
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
