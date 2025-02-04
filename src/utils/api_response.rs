use actix_web::{
    body::{self, BoxBody},
    http::StatusCode,
    web, HttpResponse, Responder, ResponseError,
};
use std::fmt::{self, Display};

#[derive(Debug)]
pub struct ApiResponse {
    pub status_code: u16,
    pub body: String,
    pub response_code: StatusCode,
}

impl ApiResponse {
    pub fn new(status_code: u16, body: String) -> Self {
        ApiResponse {
            status_code,
            body,
            response_code: StatusCode::from_u16(status_code).unwrap(),
        }
    }
}

// Responder 트레잇을 구현하여 Actix에서 사용 가능하도록 설정
impl Responder for ApiResponse {
    type Body = BoxBody;

    fn respond_to(self, req: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        let body: BoxBody = BoxBody::new(web::BytesMut::from(self.body.as_bytes()));
        HttpResponse::new(self.response_code).set_body(body)
    }
}

// Display 트레잇 구현
impl Display for ApiResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error: {}", self.body)
    }
}

impl ResponseError for ApiResponse {
    fn error_response(&self) -> HttpResponse<BoxBody> {
        let body: BoxBody = BoxBody::new(web::BytesMut::from(self.body.as_bytes()));
        HttpResponse::new(self.response_code).set_body(body)
    }
}
