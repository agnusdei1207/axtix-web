use crate::utils::{api_response, app_state, jwt::Claims};
use actix_web::{get, patch, web};
use sea_orm::EntityTrait;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct UpdateUserModel {
    name: String,
}

#[get("")]
pub async fn user(
    app_state: web::Data<app_state::AppState>,
    claim_data: Claims,
) -> Result<api_response::ApiResponse, api_response::ApiResponse> {
    let user_model = entity::user::Entity::find_by_id(claim_data.id)
        .one(&app_state.db)
        .await
        .map_err(|err| api_response::ApiResponse::new(500, err.to_string()))?
        .ok_or(api_response::ApiResponse::new(
            404,
            "User Not Found".to_owned(),
        ))?;

    Ok(api_response::ApiResponse::new(
        200,
        format!(
            " {{ 'name': '{}', 'email': '{}' }} ",
            user_model.name, user_model.email
        ),
    ))
}

#[patch("update")]
pub async fn update_user(
    app_state: web::Data<app_state::AppState>,
    user_data: web::Json<UpdateUserModel>,
    claim_data: Claims,
) -> Result<api_response::ApiResponse, api_response::ApiResponse> {
    Ok(api_response::ApiResponse::new(
        200,
        "user updated".to_owned(),
    ))
}
