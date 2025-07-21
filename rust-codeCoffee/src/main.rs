use axum::{routing::{get,post},Json, Router, response::IntoResponse,};         // Server는 더 이상 import 불필요
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // 1) 핸들러 정의
    async fn root() -> &'static str { "Hello, axum 0.8.4!" }

    // 2) 라우터 설정
    let app = Router::new().route("/", get(root));

    // 3) 리스너 바인딩
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080")
        .await
        .unwrap();

    // 4) axum::serve 호출
    axum::serve(listener, app).await.unwrap();
}
