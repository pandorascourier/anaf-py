use serde::{Deserialize, Serialize};

/// API Response structure.
/// 
/// V9 API returns only `found` and `notFound` fields.
/// V7/V8 APIs also include `cod` and `message` fields.
#[derive(Debug, Deserialize, Serialize)]
pub struct ApiResponse<T> {
    #[serde(alias = "cod", default)]
    pub status: usize,

    #[serde(alias = "message", default)]
    pub message: String,

    #[serde(alias = "found")]
    pub data: Vec<T>,

    #[serde(alias = "notFound", default)]
    pub not_found: Vec<usize>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AsyncApiResponse<T> {
    #[serde(alias = "cod")]
    pub status: usize,

    #[serde(alias = "message")]
    pub message: String,

    #[serde(alias = "correlationId")]
    pub token: T,
}
