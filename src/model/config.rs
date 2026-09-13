use serde::{Deserialize, Serialize};

#[derive(Debug,Default,Deserialize,Serialize)]
pub struct Config{
    pub llm:LLM,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_dir:Option<String>,
}

#[derive(Debug,Default,Deserialize,Serialize)]
pub struct LLM{
    pub provider:String,
    pub base_url:String,
    pub api_key:String,
}

