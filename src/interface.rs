use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct GetFullUrl {
    pub url: String
}

#[derive(Debug, Serialize)]
pub struct ShortUrlResponse {
    pub url: String
}