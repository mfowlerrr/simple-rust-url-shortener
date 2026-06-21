use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema, Debug)]
pub struct ShortenRequest {
    pub url: String,
}

#[derive(Serialize, ToSchema, Debug)]
pub struct ShortenReply {
    pub code: String,
    pub url: String,
}

#[derive(Serialize, ToSchema, Debug)]
pub struct GetReply {
    pub url: String,
}
