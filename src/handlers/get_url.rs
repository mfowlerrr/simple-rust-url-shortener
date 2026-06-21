

use axum::{
    Json, extract::{Path, State}, http::StatusCode
};

use crate::{
    models::GetReply, 
    state::AppState, 
};

#[utoipa::path(
    get, 
    path = "/{url_code}", 
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

    let map = state.urls.lock().await;

    let url = match map.get(&url_code) {
        Some(x) => x, 
        None => return Err(StatusCode::NOT_FOUND), 
    };

    Ok(Json(GetReply { url: url.clone()}))
}
