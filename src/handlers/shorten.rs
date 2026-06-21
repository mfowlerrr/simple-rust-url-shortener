
use axum::{
    extract::State, 
    Json,
};

use crate::{
    models::{ShortenReply, ShortenRequest}, 
    state::AppState, 
    utils::generate_code, 
};

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
    let mut map = state.urls.lock().await;

    let code: String = loop {
        let candidate = generate_code();
        if !map.contains_key(&candidate){
            break candidate; 
        }
    };

    map.insert(code.clone(), payload.url);

    Json(ShortenReply {
        code: code.clone(),
        url: format!("http://localhost:3000/{}", code),
    })
}
