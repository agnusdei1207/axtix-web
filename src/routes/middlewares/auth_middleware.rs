use actix_web::{
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    http::header::HeaderValue,
    middleware::Next,
    Error, HttpMessage,
};
use log::error;

use crate::utils::{
    api_response,
    jwt::{decode_jwt, Claims},
};

/// 인증 미들웨어: HTTP 요청의 Authorization 헤더를 검사하여 JWT 토큰을 검증
pub async fn check_auth_middleware(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    // 1. Authorization 헤더 확인
    let auth_header: Option<&HeaderValue> = req.headers().get("Authorization");

    let auth_value = match auth_header {
        Some(value) => value.to_str().ok(),
        None => None,
    };

    // Authorization 헤더가 없거나 문자열 변환 실패 시 401 Unauthorized 반환
    let token = match auth_value {
        Some(auth) if auth.starts_with("Bearer ") => auth[7..].to_string(), // "Bearer " 제거
        _ => {
            return Err(Error::from(api_response::ApiResponse::new(
                401,
                "Unauthorized: Missing or invalid token".to_string(),
            )))
        }
    };

    let decoded_jwt: Claims = match decode_jwt(token) {
        Ok(claims) => {
            req.extensions_mut().insert(claims.clone());
            claims
        }
        Err(err) => {
            error!("JWT decoding failed: {}", err);
            return Err(Error::from(api_response::ApiResponse::new(
                401,
                "Unauthorized: Invalid token".to_string(),
            )));
        }
    };

    // 3. 미들웨어 체인 실행 (다음 핸들러 호출)
    next.call(req).await.map_err(|err: Error| {
        error!("Middleware execution failed: {}", err);
        Error::from(api_response::ApiResponse::new(500, err.to_string()))
    })
}
