// `crate::utils` 모듈에서 `api_response`와 `app_state::AppState`를 가져옵니다.
// `api_response`는 API 응답을 표준화하는 유틸리티이고, `AppState`는 애플리케이션의 상태(예: 데이터베이스 연결)를 관리합니다.
use crate::utils::{api_response, app_state::AppState};

use sea_orm::ColumnTrait;
use sea_orm::Condition;
// `actix_web` 크레이트에서 필요한 항목들을 가져옵니다.
// `post`는 POST 요청을 처리하는 핸들러를 정의할 때 사용하고,
// `web`은 요청 데이터를 추출하거나 상태를 관리하는 데 사용됩니다.
// `Responder`는 HTTP 응답을 생성하는 트레이트입니다.
use actix_web::{post, web, Responder};

// `sea_orm` 크레이트에서 `ActiveModelTrait`와 `Set`을 가져옵니다.
// `ActiveModelTrait`는 데이터베이스 모델을 조작하는 메서드를 제공하고,
// `Set`은 필드 값을 설정할 때 사용됩니다.
use sea_orm::ActiveModelTrait;
use sea_orm::EntityTrait;
use sea_orm::QueryFilter;
use sea_orm::Set;

// `serde` 크레이트에서 `Deserialize`를 가져옵니다.
// `Deserialize`는 JSON 데이터를 Rust 구조체로 변환할 때 사용됩니다.
use serde::Deserialize;

// `RegisterModel` 구조체는 클라이언트로부터 받은 JSON 데이터를 표현합니다.
// `Deserialize`를 구현하여 JSON 데이터를 이 구조체로 변환할 수 있습니다.
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
