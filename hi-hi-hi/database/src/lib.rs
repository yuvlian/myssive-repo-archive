use sqlx::SqlitePool;
use std::env;
use std::sync::Arc;

pub mod client_data;

pub type Arclite = Arc<SqlitePool>;

pub async fn init_sqlite_pool() -> Arclite {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = SqlitePool::connect(&db_url)
        .await
        .expect("Failed to create pool");

    Arc::new(pool)
}
