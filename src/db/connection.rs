use std::time::Duration;

use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

#[derive(Clone)]
pub struct Connection {}

impl Connection {
    pub async fn new_pool(url: &str) -> Pool<Postgres> {
        PgPoolOptions::new()
            .max_connections(5)
            .acquire_timeout(Duration::from_secs(3))
            .connect(url)
            .await
            .expect("can't connect to database")
    }
}
