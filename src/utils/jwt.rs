use super::constants;
use actix_web::{FromRequest, HttpMessage};
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, DecodingKey, EncodingKey, Header};
use std::future;

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct Claims {
    pub exp: usize,
    pub iat: usize,
    pub email: String,
    pub id: u64,
}

impl FromRequest for Claims {
    type Error = actix_web::Error;
    type Future = future::Ready<Result<Self, Self::Error>>;

    fn from_request(
        req: &actix_web::HttpRequest,
        payload: &mut actix_web::dev::Payload,
    ) -> std::future::Ready<Result<Claims, actix_web::Error>> {
        match req.extensions().get::<Claims>() {
            Some(claim) => future::ready(Ok(claim.clone())),
            None => future::ready(Err(actix_web::error::ErrorBadRequest("bad claims"))),
        }
    }
}

pub fn encode_jwt(email: String, id: u64) -> Result<String, jsonwebtoken::errors::Error> {
    let now: chrono::DateTime<Utc> = Utc::now();
    let exp: chrono::TimeDelta = Duration::hours(24);

    let claims = Claims {
        exp: (now + exp).timestamp() as usize,
        iat: now.timestamp() as usize,
        email,
        id,
    };

    let secret: String = (*constants::JWT_SECRET).clone();

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
}

pub fn decode_jwt(jwt: String) -> Result<Claims, jsonwebtoken::errors::Error> {
    let secret = (*constants::JWT_SECRET).clone();

    let decoded_data: Result<jsonwebtoken::TokenData<Claims>, jsonwebtoken::errors::Error> =
        jsonwebtoken::decode::<Claims>(
            &jwt,
            &DecodingKey::from_secret(secret.as_bytes()),
            &jsonwebtoken::Validation::default(),
        );

    match decoded_data {
        Ok(token_data) => Ok(token_data.claims),
        Err(_) => Err(jsonwebtoken::errors::Error::from(
            jsonwebtoken::errors::ErrorKind::InvalidToken,
        )),
    }
}
