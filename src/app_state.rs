use sqlx::{Pool, Postgres};

use crate::{config::AppConfig, db::connection::Connection};

#[derive(Clone)]

pub struct AppState {
    pub db_connection: Pool<Postgres>,
}

impl AppState {
    pub async fn new(config: AppConfig) -> AppState {
        let db_connection = Connection::new_pool(&config.database.url).await;

        AppState { db_connection }
    }
}
