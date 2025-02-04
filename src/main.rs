use actix_web::{get, middleware::Logger, web, App, HttpServer, Responder};
use dotenv::dotenv;
use env_logger;

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 환경 변수 로드
    dotenv().ok();

    // 로그 레벨 설정 (RUST_LOG가 없으면 기본값 설정)
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }

    // 로거 초기화
    env_logger::init();

    // 서버 실행
    HttpServer::new(|| App::new().wrap(Logger::default()).service(greet))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
