use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(default)]
pub struct Llm {
    pub provider: String,
    pub base_url: String,
    pub api_key: String,
    pub model: String,
    pub summary_language: String,
}

impl Default for Llm {
    fn default() -> Self {
        Self {
            provider: String::new(),
            base_url: String::new(),
            api_key: String::new(),
            model: String::new(),
            summary_language: "Chinese".to_string(),
        }
    }
}
