use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema)]
pub struct ShortenRequest {
    pub url: String,
}

#[derive(Serialize, ToSchema)]
pub struct ShortenReply {
    pub code: String,
    pub short_url: String,
}

#[derive(Serialize, ToSchema)]
pub struct GetReply {
    pub url: String,
}
