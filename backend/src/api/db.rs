use sea_orm::{Database, DbConn};

pub async fn init_db() -> DbConn {
    dotenvy::dotenv().ok();
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    Database::connect(&db_url)
        .await
        .expect("Failed to connect to DB")
}
