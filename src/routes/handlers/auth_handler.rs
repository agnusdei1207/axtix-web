use crate::utils::{api_response, app_state::AppState};
use actix_web::{post, web, Responder};
use sea_orm::ActiveModelTrait;
use sea_orm::Set;
use serde::Deserialize;

#[derive(Deserialize)]
struct RegisterModel {
    name: String,
    email: String,
    password: String,
}

#[post("/register")]
pub async fn register(
    app_state: web::Data<AppState>,
    register_json: web::Json<RegisterModel>,
) -> impl Responder {
    let user_model = entity::user::ActiveModel {
        name: Set(register_json.name.clone()),
        email: Set(register_json.email.clone()),
        password: Set(register_json.password.clone()),
        ..Default::default()
    }
    .insert(&app_state.db)
    .await
    .unwrap();

    api_response::ApiResponse::new(200, format!("{}", user_model.id))
}
