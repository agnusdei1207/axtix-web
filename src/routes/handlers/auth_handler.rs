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
    let user_model: entity::user::Model = entity::user::ActiveModel {
        name: Set(data.name),
        email: Set(data.email),
        password: Set(digest(data.password)),
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
    let user: Option<entity::user::Model> = entity::user::Entity::find()
        .filter(
            Condition::all()
                .add(entity::user::Column::Email.eq(&login_json.email))
                .add(entity::user::Column::Password.eq(digest(&login_json.password))),
        )
        .one(&app_state.db)
        .await
        .unwrap();

    dbg!(&user);

    if user.is_none() {
        return api_response::ApiResponse::new(404, "User not found".to_string());
    }

    api_response::ApiResponse::new(200, user.unwrap().name)
}
