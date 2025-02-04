// `crate::utils` 모듈에서 `api_response`와 `app_state::AppState`를 가져옵니다.
// `api_response`는 API 응답을 표준화하는 유틸리티이고, `AppState`는 애플리케이션의 상태(예: 데이터베이스 연결)를 관리합니다.
use crate::utils::{api_response, app_state::AppState};

// `actix_web` 크레이트에서 필요한 항목들을 가져옵니다.
// `post`는 POST 요청을 처리하는 핸들러를 정의할 때 사용하고,
// `web`은 요청 데이터를 추출하거나 상태를 관리하는 데 사용됩니다.
// `Responder`는 HTTP 응답을 생성하는 트레이트입니다.
use actix_web::{post, web, Responder};

// `sea_orm` 크레이트에서 `ActiveModelTrait`와 `Set`을 가져옵니다.
// `ActiveModelTrait`는 데이터베이스 모델을 조작하는 메서드를 제공하고,
// `Set`은 필드 값을 설정할 때 사용됩니다.
use sea_orm::ActiveModelTrait;
use sea_orm::Set;

// `serde` 크레이트에서 `Deserialize`를 가져옵니다.
// `Deserialize`는 JSON 데이터를 Rust 구조체로 변환할 때 사용됩니다.
use serde::Deserialize;

// `RegisterModel` 구조체는 클라이언트로부터 받은 JSON 데이터를 표현합니다.
// `Deserialize`를 구현하여 JSON 데이터를 이 구조체로 변환할 수 있습니다.
#[derive(Deserialize)]
struct RegisterModel {
    name: String,     // 사용자의 이름
    email: String,    // 사용자의 이메일
    password: String, // 사용자의 비밀번호
}

// `/register` 경로로 들어오는 POST 요청을 처리하는 핸들러입니다.
#[post("/register")]
pub async fn register(
    app_state: web::Data<AppState>, // 애플리케이션 상태(데이터베이스 연결 등)를 주입받습니다.
    register_json: web::Json<RegisterModel>, // 클라이언트로부터 받은 JSON 데이터를 `RegisterModel`로 변환합니다.
) -> impl Responder {
    // `RegisterModel` 데이터를 기반으로 `user::ActiveModel`을 생성합니다.
    // `Set`을 사용하여 각 필드의 값을 설정합니다.
    let user_model = entity::user::ActiveModel {
        name: Set(register_json.name.clone()),         // 이름 설정
        email: Set(register_json.email.clone()),       // 이메일 설정
        password: Set(register_json.password.clone()), // 비밀번호 설정
        ..Default::default()                           // 나머지 필드는 기본값으로 설정
    }
    // 데이터베이스에 새로운 사용자를 삽입합니다.
    .insert(&app_state.db)
    .await
    // 삽입 작업이 실패하면 패닉을 발생시킵니다. (실제 프로덕션 코드에서는 에러 처리를 권장합니다.)
    .unwrap();

    // 성공적으로 사용자가 생성되면, `api_response::ApiResponse`를 사용하여 응답을 반환합니다.
    // HTTP 상태 코드 200과 생성된 사용자의 ID를 반환합니다.
    api_response::ApiResponse::new(200, format!("{}", user_model.id))
}
