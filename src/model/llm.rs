use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Llm {
    pub provider: String,
    pub base_url: String,
    pub api_key: String,
    pub model: String,
}
