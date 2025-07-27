use axum::{routing::{get,post},Json, Router, response::IntoResponse,};         // Server는 더 이상 import 불필요
use std::net::SocketAddr;
use rust_codeCoffee::db::connection::establish_connection;
#[tokio::main]
async fn main() {

    let dbcon = establish_connection().await.expect("db 연결 중 치명적 에러 발생!");
    // 1) 핸들러 정의
    async fn root() -> &'static str { "Hello, axum 0.8.4!" }

    // 2) 라우터 설정
    let app = Router::new().route("/", get(root));

    // 3) 리스너 바인딩
    let listener = tokio::net::TcpListener::bind("localhost:8080")
        .await
        .unwrap();

    // 4) axum::serve 호출
    axum::serve(listener, app).await.expect("서버 실행중 치명적 에러 발생!"); // unwrap() 대신 서버 패닉 + 메세지 호출로 변경 

    //운영때 위 대신 사용하는 것
    // };// let contents = match std::fs::read_to_string("config.toml") {
    //     Ok(s) => s,
    //     Err(e) => {
    //         tracing::error!("config.toml 파일 읽기 실패: {}", e);
    //         return Err(AppError::ConfigLoadFailed);
    //     }
}
