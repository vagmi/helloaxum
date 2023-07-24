use sqlx::PgPool;
use crate::error::Result;

#[derive(Debug, Clone)]
pub struct AppState {
    pub pool: PgPool
}

impl AppState {
    pub async fn new() -> Result<AppState> {
        let conn_str = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let pool = PgPool::connect(&conn_str).await?;
        sqlx::migrate!("./migrations").run(&pool).await?;
        Ok(AppState {
            pool
        })
    }
}
