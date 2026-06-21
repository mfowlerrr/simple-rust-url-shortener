use axum::{
    Json, extract::{Path, State}, http::StatusCode
};
use tracing::{debug, error};

use crate::{
    models::{GetReply, ShortenReply}, 
    state::{AppState, UrlStoreTrait}, 
};

#[tracing::instrument]
#[utoipa::path(
    get, 
    path = "/url/{url_code}", 
    params(
        ("url_code" = String, Path, description="Short URL code")

    ), 
    responses( 
        (status = 200, body=GetReply), 
        (status = 404, description="Url Not Found")
    )

)]
pub async fn get_url(
    State(state): State<AppState>,
    Path(url_code): Path<String> 
) -> Result<Json<GetReply>, StatusCode> {
    debug!("Get by URL called");

    // let map = state.store.map.lock().await;

    let url = match state.store.get(&url_code).await {
        Some(x) => x, 
        None =>  {
            error!("No url is stored with the provided code");
            return Err(StatusCode::NOT_FOUND)}
    };

    Ok(Json(GetReply { url: url.clone()}))
}


#[utoipa::path(
    get, 
    path = "/url", 
    responses( 
        (status = 200, body=Vec<ShortenReply>), 
    )

)]
pub async fn get_urls(
    State(state): State<AppState>,
) -> Result<Json<Vec<ShortenReply>>, StatusCode> {
    debug!("Get ALL URLs called");

    Ok(Json(state.store.get_all().await))
}
