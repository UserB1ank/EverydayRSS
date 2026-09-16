use serde::{Deserialize, Serialize};

#[derive(Debug,Default,Deserialize,Serialize)]
pub struct LLM{
    pub provider:String,
    pub base_url:String,
    pub api_key:String,
    pub model:String,
}