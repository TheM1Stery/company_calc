use sqlx::sqlite::{SqliteConnectOptions, SqlitePool};
use sqlx::Result;

pub async fn get_pooled_connection(filename: &str) -> Result<SqlitePool> {
    let options = SqliteConnectOptions::new()
        .filename(filename)
        .create_if_missing(true);

    SqlitePool::connect_with(options).await
}
