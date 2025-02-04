use actix_web::{
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    middleware::Next,
    Error,
};
use jsonwebtoken::TokenData;

use crate::utils::{
    api_response,
    jwt::{decode_jwt, Claims},
};

pub async fn check_auth_middleware(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    let auth: Option<&actix_web::http::header::HeaderValue> = req.headers().get("Authorization");

    if auth.is_none() {
        return Err(Error::from(api_response::ApiResponse::new(
            401,
            "Unauthorized".to_string(),
        )));
    }

    let token: String = auth.unwrap().to_str().unwrap().to_owned();
    let decoded_jwt: Claims = decode_jwt(token).unwrap();

    next.call(req)
        .await
        .map_err(|err: Error| Error::from(api_response::ApiResponse::new(500, err.to_string())))
}
