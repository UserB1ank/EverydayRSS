use serde::{Deserialize, Serialize};

#[derive(Debug,Deserialize,Serialize)]
pub struct Config{
    pub llm:LLM,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_dir:Option<String>,
    pub proxy:Option<String>,
    pub timeout:Option<u32>,
    pub loglevel:String,
}

impl Default for Config{
    fn default() -> Self {
        Config{
            llm:LLM::default(),
            resource_dir:None,
            proxy:None,
            timeout:Some(3),
            loglevel:"INFO".into(),
        }
    }
}

#[derive(Debug,Default,Deserialize,Serialize)]
pub struct LLM{
    pub provider:String,
    pub base_url:String,
    pub api_key:String,
}

