use actix_web::{
    body::{self, BoxBody}, // HTTP 응답 바디 타입을 다루는 모듈 (BoxBody는 동적 크기 데이터를 담기 위한 래퍼)
    http::StatusCode,      // HTTP 상태 코드를 다루기 위한 모듈 (예: 200 OK, 404 Not Found 등)
    web,                   // 웹 관련 모듈 (BytesMut을 사용하여 바디 데이터를 다룰 수 있음)
    HttpResponse,          // HTTP 응답 객체를 생성하는 모듈
    Responder,             // Actix 웹 응답 트레잇 (사용자 정의 응답 타입을 만들 때 필요)
};

// API 응답을 표현하는 구조체
pub struct ApiResponse {
    pub status_code: u16,          // 응답 상태 코드 (예: 200, 404 등 HTTP 코드)
    pub body: String,              // 응답 본문 (클라이언트에 전달할 메시지, JSON 등)
    pub response_code: StatusCode, // Actix의 StatusCode 타입을 사용하여 상태 코드 저장
}

impl ApiResponse {
    // 새로운 ApiResponse 객체를 생성하는 생성자 함수
    pub fn new(status_code: u16, body: String) -> Self {
        ApiResponse {
            status_code,                                               // 상태 코드 저장 (u16 타입)
            body,                                                      // 응답 본문 저장
            response_code: StatusCode::from_u16(status_code).unwrap(), // u16 값을 StatusCode로 변환 (정상적인 HTTP 코드인지 검증됨)
        }
    }
}

// Responder 트레잇을 구현하여 Actix에서 사용 가능하도록 설정
impl Responder for ApiResponse {
    type Body = BoxBody; // HTTP 응답 바디의 타입 정의 (BoxBody는 효율적인 응답 관리를 위해 사용)

    fn respond_to(self, req: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        // 클라이언트 요청(req)을 기반으로 응답을 생성 (req는 사용되지 않지만 확장 가능)
        let body: BoxBody = BoxBody::new(web::BytesMut::from(self.body.as_bytes())); // String을 바이트 버퍼로 변환하여 BoxBody로 래핑
        HttpResponse::new(self.response_code).set_body(body) // HTTP 응답 객체를 생성하고 변환된 바디를 설정하여 반환
    }
}
