use serde::{Deserialize, Serialize};

pub const DEFAULT_ADDR: &str = "127.0.0.1:8080";

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatRequest {
    pub prompt: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatResponse {
    pub response: String,
}
