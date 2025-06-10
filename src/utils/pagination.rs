use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct PaginatedResponse<T> {
    pub count: i64,
    pub results: Vec<T>,
    pub limit: i64,
    pub offset: i64,
}

#[derive(Debug, Deserialize)]
pub struct PaginationParams {
    #[serde(default = "default_limit")]
    pub limit: i64,

    #[serde(default = "default_offset")]
    pub offset: i64,
}

fn default_limit() -> i64 {
    10
}

fn default_offset() -> i64 {
    0
}
