use actix_web::{
    body::{self, BoxBody}, // HTTP 응답 바디 타입을 다루는 모듈
    http::StatusCode,      // HTTP 상태 코드를 다루기 위한 모듈
    web,
    HttpResponse,
    Responder, // Actix 웹 모듈
};

// API 응답을 표현하는 구조체
pub struct ApiResponse {
    pub status_code: u16,          // 응답 상태 코드 (예: 200, 404 등)
    pub body: String,              // 응답 본문
    pub response_code: StatusCode, // Actix의 StatusCode 타입을 사용한 상태 코드
}

impl ApiResponse {
    // 새로운 ApiResponse 객체 생성
    pub fn new(status_code: u16, body: String) -> Self {
        ApiResponse {
            status_code,
            body,
            response_code: StatusCode::from_u16(status_code).unwrap(), // u16 값을 StatusCode로 변환
        }
    }
}

// Responder 트레잇을 구현하여 Actix에서 사용할 수 있도록 설정
impl Responder for ApiResponse {
    type Body = BoxBody; // HTTP 응답 바디의 타입 정의

    fn respond_to(self, req: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        let body: BoxBody = BoxBody::new(web::BytesMut::from(self.body.as_bytes())); // 응답 바디 변환
        HttpResponse::new(self.response_code).set_body(body) // HTTP 응답 객체 생성 및 반환
    }
}
