use sea_orm::{Database, DatabaseConnection, DbErr};
use dotenvy::dotenv;
use std::env;

/// .env 로드 후 DATABASE_URL로 DB 풀을 생성해서 반환
pub async fn establish_connection() -> Result<DatabaseConnection, DbErr> {
    println!("db 연결 시작");

    // dotenv() 결과를 받아서 성공/실패 여부 찍기
    let opt = dotenv().ok();
    if opt.is_some() {
        println!("env 파일 로드 성공");
    } else {
        println!("env 파일 로드 실패 (환경변수가 이미 설정되었거나 .env가 없음)");
    }

    // DATABASE_URL 환경변수 읽기
    let database_url = env::var("DATABASE_URL")
        .expect(".env DATABASE_URL 환경변수 읽기 실패");

    // 연결 시도, 실패 시 Err(DbErr) 반환
    let db = Database::connect(&database_url).await?;
    println!("db 연결 성공");

    Ok(db)
}
