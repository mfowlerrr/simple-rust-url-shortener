
use axum::{
    extract::State, 
    Json,
};
use tracing::debug; 

use crate::{
    models::{ShortenReply, ShortenRequest}, 
    state::{AppState, UrlStoreTrait},
    utils::generate_code, 
};

#[tracing::instrument]
#[utoipa::path(
    post, 
    path = "/url", 
    request_body =ShortenRequest, 
    responses( 
        (status = 200, body=ShortenReply)
    )

)]
pub async fn shorten(
    State(state): State<AppState>,
    Json(payload): Json<ShortenRequest>,
) -> Json<ShortenReply> {
    debug!("Shorten called.");

    let code: String = loop {
        let candidate = generate_code();
        match state.store.get(&candidate).await {
            None => break candidate, 
            Some(_) => continue
        };
    };

    state.store.insert(code.clone(), payload.url).await;

    Json(ShortenReply {
        code: code.clone(),
        url: format!("http://localhost:3000/{}", code),
    })
}
