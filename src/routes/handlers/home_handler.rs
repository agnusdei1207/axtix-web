use actix_web::{get, web, Responder};
use sea_orm::{ConnectionTrait, QueryResult, Statement};

use crate::utils::{api_response, app_state::AppState};

#[get("/hello/{name}")]
pub async fn greet(name: web::Path<String>) -> impl Responder {
    api_response::ApiResponse::new(200, format!("Hello {}!", name))
}

#[get("/test")]
pub async fn test(app_state: web::Data<AppState>) -> impl Responder {
    let res: Vec<QueryResult> = app_state
        .db
        .query_all(Statement::from_string(
            sea_orm::DatabaseBackend::Postgres,
            "SELECT * FROM user;",
        ))
        .await
        .unwrap();
    api_response::ApiResponse::new(200, "Test".to_string())
}
