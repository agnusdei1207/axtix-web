use actix_web::{middleware::Logger, web, App, HttpServer};
use dotenv::dotenv;
use env_logger;
use migration::{Migrator, MigratorTrait};
use sea_orm::{Database, DatabaseConnection};
use utils::app_state::AppState;

mod routes;
mod utils;

#[derive(Debug)]
struct MainError {
    message: String,
}

#[actix_web::main] // Actix의 비동기 런타임 매크로 (메인 함수가 비동기로 실행됨)
async fn main() -> Result<(), MainError> {
    // 환경 변수 RUST_LOG가 설정되지 않았다면 기본값을 "actix_web=info"로 설정
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }

    // .env 파일을 로드하여 환경 변수 사용 가능하게 설정
    dotenv().ok();
    // env_logger 초기화 (로그 시스템 활성화)
    env_logger::init();

    let address: String = (utils::constants::ADDRESS).clone();
    let port: u16 = (utils::constants::PORT).clone();
    let database_url: String = (utils::constants::DATABASE_URL).clone();
    let db_connection: DatabaseConnection =
        Database::connect(database_url)
            .await
            .map_err(|err| MainError {
                message: err.to_string(),
            })?;

    Migrator::up(&db_connection, None)
        .await
        .map_err(|err| MainError {
            message: err.to_string(),
        })?;

    // HTTP 서버 생성 및 실행
    HttpServer::new(move || {
        // 새로운 Actix 웹 애플리케이션 인스턴스를 생성
        App::new()
            .app_data(web::Data::new(AppState {
                db: db_connection.clone(),
            }))
            // 로깅 미들웨어 추가 (요청 정보를 로그로 남김)
            .wrap(Logger::default())
            .configure(routes::home_routes::config)
            .configure(routes::auth_routes::config)
            .configure(routes::user_routes::config)
    })
    .bind((address, port))
    .map_err(|err| MainError {
        message: err.to_string(),
    })?
    .run()
    .await
    .map_err(|err| MainError {
        message: err.to_string(),
    })
}
