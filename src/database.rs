use sqlx::sqlite::SqlitePool;
use sqlx::Result;

pub async fn get_pooled_connection(connection_str: &str) -> Result<SqlitePool> {
    SqlitePool::connect(connection_str).await
}
