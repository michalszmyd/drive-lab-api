use actions::users;
use app_state::AppState;
use axum::{
    routing::{get, post},
    Router,
};
use config::AppConfig;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub mod actions;
pub mod app_state;
pub mod config;
pub mod db;

const BINDING_LISTEN_HOST: &str = "0.0.0.0:3004";

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                "example_parse_body_based_on_content_type=debug,tower_http=debug".into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = AppConfig::new();
    let app_state = AppState::new(config).await;

    let app = Router::new()
        .route("/", get(|| async { "OK" }))
        .route("/users", get(users::new))
        .route("/users", post(users::create))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind(BINDING_LISTEN_HOST)
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}
