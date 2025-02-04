// `api_response`는 API 응답을 표준화하는 유틸리티이고, `AppState`는 애플리케이션의 상태(예: 데이터베이스 연결)를 관리합니다.
use crate::utils::{api_response, app_state::AppState};

use sea_orm::ColumnTrait;
use sea_orm::Condition;
// `Responder`는 HTTP 응답을 생성하는 트레이트입니다.
use actix_web::{post, web, Responder};

// `Set`은 필드 값을 설정할 때 사용됩니다.
use sea_orm::ActiveModelTrait;
use sea_orm::EntityTrait;
use sea_orm::QueryFilter;
use sea_orm::Set;

// `Deserialize`는 JSON 데이터를 Rust 구조체로 변환할 때 사용됩니다.
use serde::Deserialize;

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
    let user_model: entity::user::Model = entity::user::ActiveModel {
        name: Set(data.name),
        email: Set(data.email),
        password: Set(data.password),
        ..Default::default()
    }
    .insert(&app_state.db)
    .await
    .unwrap();

    api_response::ApiResponse::new(200, format!("{}", user_model.id))
}

#[post("/login")]
pub async fn login(
    app_state: web::Data<AppState>,
    login_json: web::Json<LoginModel>,
) -> impl Responder {
    let user = entity::user::Entity::find()
        .filter(
            Condition::all()
                .add(entity::user::Column::Email.eq(&login_json.email))
                .add(entity::user::Column::Password.eq(&login_json.password)),
        )
        .one(&app_state.db)
        .await
        .unwrap();

    if user.is_none() {
        return api_response::ApiResponse::new(404, "User not found".to_string());
    }

    api_response::ApiResponse::new(200, "".to_string())
}
