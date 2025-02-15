use crate::utils::{api_response, app_state, jwt::Claims};
use actix_web::{get, web, Error, HttpResponse, Responder};
use sea_orm::EntityTrait;
use serde_json::json;

#[get("")]
pub async fn user(
    app_state: web::Data<app_state::AppState>,
    claim_data: Claims,
) -> Result<HttpResponse, Error> {
    let user_model = entity::user::Entity::find_by_id(claim_data.id)
        .one(&app_state.db)
        .await
        .map_err(|err| actix_web::error::ErrorInternalServerError(err.to_string()))?
        .ok_or_else(|| actix_web::error::ErrorNotFound("User not found"))?;

    let user_json = json!({
        "id": user_model.id,
        "name": user_model.name,
        "email": user_model.email
    });

    Ok(HttpResponse::Ok().json(user_json))
}
