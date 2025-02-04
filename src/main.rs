use actix_web::{get, middleware::Logger, web, App, HttpServer, Responder};
use dotenv::dotenv;
use env_logger;

mod routes;
mod utils;

// GET 요청을 처리하는 핸들러 함수
// 요청 URL: /hello/{name} -> {name} 값이 동적으로 전달됨
#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    // 클라이언트에게 "Hello {name}!" 형식의 문자열을 응답으로 보냄
    format!("Hello {name}!")
}

#[actix_web::main] // Actix의 비동기 런타임 매크로 (메인 함수가 비동기로 실행됨)
async fn main() -> std::io::Result<()> {
    // .env 파일을 로드하여 환경 변수 사용 가능하게 설정
    dotenv().ok();

    // 환경 변수 RUST_LOG가 설정되지 않았다면 기본값을 "actix_web=info"로 설정
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }

    // env_logger 초기화 (로그 시스템 활성화)
    env_logger::init();

    // HTTP 서버 생성 및 실행
    HttpServer::new(|| {
        // 새로운 Actix 웹 애플리케이션 인스턴스를 생성
        App::new()
            // 로깅 미들웨어 추가 (요청 정보를 로그로 남김)
            .wrap(Logger::default())
            // "greet" 핸들러를 서비스로 등록
            .service(greet)
    })
    // 서버를 127.0.0.1(로컬호스트) 8080번 포트에 바인딩
    .bind(("127.0.0.1", 8080))?
    // 서버 실행 (비동기 처리)
    .run()
    .await
}
