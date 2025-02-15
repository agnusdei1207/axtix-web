use super::constants;
use actix_web::{FromRequest, HttpMessage};
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, DecodingKey, EncodingKey, Header};
use std::future;

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct Claims {
    pub exp: usize,    // 토큰 만료 시간 (Expiration Time)
    pub iat: usize,    // 토큰 발급 시간 (Issued At)
    pub email: String, // 사용자 이메일
    pub id: u64,       // 사용자 ID
}

impl FromRequest for Claims {
    type Error = actix_web::Error; // 오류 타입 정의
    type Future = future::Ready<Result<Self, Self::Error>>; // 비동기 작업의 결과 타입 정의

    fn from_request(
        req: &actix_web::HttpRequest,          // 현재 HTTP 요청 객체
        payload: &mut actix_web::dev::Payload, // 요청 본문 (여기서는 사용하지 않음)
    ) -> std::future::Ready<Result<Claims, actix_web::Error>> {
        // 요청의 확장 데이터에서 Claims 타입의 데이터를 가져옵니다.
        match req.extensions().get::<Claims>() {
            // 확장 데이터에서 Claims를 성공적으로 가져온 경우, 복제본을 반환합니다.
            Some(claim) => future::ready(Ok(claim.clone())),
            // 확장 데이터에 Claims가 없는 경우, 오류를 반환합니다.
            None => future::ready(Err(actix_web::error::ErrorBadRequest("bad claims"))),
        }
    }
}

pub fn encode_jwt(email: String, id: u64) -> Result<String, jsonwebtoken::errors::Error> {
    let now: chrono::DateTime<Utc> = Utc::now(); // 현재 시간
    let exp: chrono::TimeDelta = Duration::hours(24); // 토큰 만료 시간 (24시간)

    // Claims 구조체 생성
    let claims = Claims {
        exp: (now + exp).timestamp() as usize, // 현재 시간 + 24시간
        iat: now.timestamp() as usize,         // 현재 시간
        email,                                 // 사용자 이메일
        id,                                    // 사용자 ID
    };

    let secret: String = (*constants::JWT_SECRET).clone(); // JWT 시크릿 키

    // JWT 토큰 생성
    encode(
        &Header::default(),                         // JWT 헤더 (기본값 사용)
        &claims,                                    // 페이로드 (Claims 구조체)
        &EncodingKey::from_secret(secret.as_ref()), // 시크릿 키로 인코딩
    )
}

pub fn decode_jwt(jwt: String) -> Result<Claims, jsonwebtoken::errors::Error> {
    let secret = (*constants::JWT_SECRET).clone(); // JWT 시크릿 키

    // JWT 토큰 디코딩
    let decoded_data: Result<jsonwebtoken::TokenData<Claims>, jsonwebtoken::errors::Error> =
        jsonwebtoken::decode::<Claims>(
            &jwt,                                         // 디코딩할 JWT 토큰
            &DecodingKey::from_secret(secret.as_bytes()), // 시크릿 키로 디코딩
            &jsonwebtoken::Validation::default(),         // 기본 검증 옵션 사용
        );

    // 디코딩 결과 처리
    match decoded_data {
        Ok(token_data) => Ok(token_data.claims), // 디코딩 성공 시 Claims 반환
        Err(_) => Err(jsonwebtoken::errors::Error::from(
            jsonwebtoken::errors::ErrorKind::InvalidToken, // 디코딩 실패 시 오류 반환
        )),
    }
}
