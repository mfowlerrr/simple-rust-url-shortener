mod handlers;
mod models;
mod state;
mod utils;

use crate::handlers::{
    get_url::{__path_get_url, __path_get_urls, get_url, get_urls},
    shorten::{__path_shorten, shorten},
};
use crate::state::{AppState, InMemoryStore};
use tokio::net::TcpListener;
use tracing::info;

#[tokio::main]
async fn main() {
    // setup logging tracing
    tracing_subscriber::fmt::init();

    let shared_state: AppState = AppState {
        store: InMemoryStore::new(),
    };

    let (app, api) = utoipa_axum::router::OpenApiRouter::new()
        .routes(utoipa_axum::routes!(shorten))
        .routes(utoipa_axum::routes!(get_url))
        .routes(utoipa_axum::routes!(get_urls))
        .split_for_parts();

    let app = app
        .merge(utoipa_swagger_ui::SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", api))
        .with_state(shared_state);

    let host = String::from("0.0.0.0");
    let port = 3000;
    let addr = format!("{}:{}", host, port);

    let listener: TcpListener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    info!(addr=%addr, "Starting Server");
    axum::serve(listener, app).await.unwrap();
}
