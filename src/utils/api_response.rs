use actix_web::http::StatusCode;
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
