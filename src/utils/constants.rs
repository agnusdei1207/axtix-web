use lazy_static::lazy_static;
use std::env;

lazy_static! {
    // 환경 변수 "ADDRESS"를 읽어와서 정적 변수로 저장
    pub static ref ADDRESS: String = set_address();
    // 환경 변수 "PORT"를 읽어와서 정적 변수로 저장
    pub static ref PORT: u16 = set_port();
}

fn set_address() -> String {
    // .env 파일 로드 (환경 변수 설정)
    dotenv::dotenv().ok(); // dotenv::dotenv()가 실패해도 프로그램이 멈추지 않도록 처리
    env::var("ADDRESS").unwrap() // 환경 변수 "ADDRESS" 값을 가져오되, 없으면 패닉 발생
}

fn set_port() -> u16 {
    dotenv::dotenv().ok(); // .env 파일 로드 (한 번 더 호출되지만, 영향 없음)
    env::var("PORT")
        .unwrap() // 환경 변수 "PORT" 값을 가져오되, 없으면 패닉 발생
        .parse::<u16>() // 문자열을 u16 정수형으로 변환
        .unwrap() // 변환 실패 시 패닉 발생 (예: PORT 값이 숫자가 아닐 경우)
}
