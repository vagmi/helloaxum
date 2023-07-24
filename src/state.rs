use serde::{Serialize, Deserialize};
use sqlx::PgPool;


#[derive(Debug, Clone)]
pub struct AppState {
    pub pool: PgPool
}

impl AppState {
    pub async fn new() -> AppState {
        let conn_str = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let pool = PgPool::connect(&conn_str).await.unwrap();
        sqlx::migrate!("./migrations").run(&pool).await.unwrap();
        AppState {
            pool
        }
    }
}
