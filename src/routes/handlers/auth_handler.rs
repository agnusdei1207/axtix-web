use crate::utils;
// `api_response`는 API 응답을 표준화하는 유틸리티이고, `AppState`는 애플리케이션의 상태(예: 데이터베이스 연결)를 관리합니다.
use crate::utils::{api_response, app_state::AppState};

use actix_web::{post, web, Responder};
use sea_orm::ActiveModelTrait;
use sea_orm::ColumnTrait;
use sea_orm::Condition;
use sea_orm::EntityTrait;
use sea_orm::QueryFilter;
use sea_orm::Set;
use serde::Deserialize;
use serde_json::json;
use sha256::digest;

#[derive(Deserialize)]
struct RegisterModel {
    name: String,
    email: String,
    password: String,
}

#[derive(Deserialize)]
struct LoginModel {
    email: String,
    password: String,
}

#[post("/register")]
pub async fn register(
    app_state: web::Data<AppState>,
    register_json: web::Json<RegisterModel>,
) -> impl Responder {
    let data: RegisterModel = register_json.into_inner();

    // 데이터베이스 삽입을 시도합니다.
    let result = entity::user::ActiveModel {
        name: Set(data.name),
        email: Set(data.email),
        password: Set(digest(data.password)),
        ..Default::default()
    }
    .insert(&app_state.db)
    .await;

    match result {
        Ok(user_model) => api_response::ApiResponse::new(200, format!("{}", user_model.id)),
        Err(e) => api_response::ApiResponse::new(500, format!("Internal server error: {}", e)),
    }
}

#[post("/login")]
pub async fn login(
    app_state: web::Data<AppState>,
    login_json: web::Json<LoginModel>,
) -> impl Responder {
    let user: Result<Option<entity::user::Model>, sea_orm::DbErr> = entity::user::Entity::find()
        .filter(
            Condition::all()
                .add(entity::user::Column::Email.eq(&login_json.email))
                .add(entity::user::Column::Password.eq(digest(&login_json.password))),
        )
        .one(&app_state.db)
        .await;

    match user {
        Ok(Some(user_data)) => {
            // 로그인 성공 시 JWT 토큰을 반환합니다.
            let token = match utils::jwt::encode_jwt(user_data.email, user_data.id) {
                Ok(token) => token,
                Err(e) => {
                    log::error!("JWT encoding failed: {:?}", e);
                    return api_response::ApiResponse::new(
                        500,
                        format!("Internal server error: {:?}", e),
                    );
                }
            };
            api_response::ApiResponse::new(200, json!({ "token": token }).to_string())
        }
        Ok(None) => {
            // 사용자 정보가 없으면 404 응답을 반환합니다.
            api_response::ApiResponse::new(404, "User not found".to_string())
        }
        Err(e) => {
            // 데이터베이스 쿼리 실패 시 500 응답을 반환합니다.
            log::error!("Database query failed: {:?}", e);
            api_response::ApiResponse::new(500, format!("Internal server error: {:?}", e))
        }
    }
}
