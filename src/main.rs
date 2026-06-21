mod handlers;
mod models;
mod state;
mod utils;

use crate::handlers::{
    get_url::{__path_get_url, __path_get_urls, get_url, get_urls},
    shorten::{__path_shorten, shorten},
};
use crate::state::AppState;
use std::{collections::HashMap, sync::Arc};
use tokio::{net::TcpListener, sync::Mutex};

#[tokio::main]
async fn main() {
    let shared_state: AppState = AppState {
        urls: Arc::new(Mutex::new(HashMap::new())),
    };

    let (app, api) = utoipa_axum::router::OpenApiRouter::new()
        .routes(utoipa_axum::routes!(shorten))
        .routes(utoipa_axum::routes!(get_url))
        .routes(utoipa_axum::routes!(get_urls))
        .split_for_parts();

    let app = app
        .merge(utoipa_swagger_ui::SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", api))
        .with_state(shared_state);

    let listener: TcpListener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
