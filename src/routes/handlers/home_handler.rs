use actix_web::{get, web, Responder};

// GET 요청을 처리하는 핸들러 함수
// 요청 URL: /hello/{name} -> {name} 값이 동적으로 전달됨
#[get("/hello/{name}")]
pub async fn greet(name: web::Path<String>) -> impl Responder {
    // 클라이언트에게 "Hello {name}!" 형식의 문자열을 응답으로 보냄
    format!("Hello {name}!")
}
