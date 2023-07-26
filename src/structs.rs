use serde::{Serialize, Deserialize};

#[derive(Serialize)]
pub struct AsciiResponse {
    pub message: String
}

#[derive(Serialize, Deserialize)]
pub struct QueryParams {
    pub image: String,
    pub width: u32,
}