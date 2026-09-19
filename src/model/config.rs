use serde::{Deserialize, Serialize};
pub(crate) use crate::model::llm::LLM;

#[derive(Debug,Deserialize,Serialize)]
pub struct Config{
    pub llm:LLM,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_dir:Option<String>,
    pub template:String,
    pub proxy:Option<String>,
    pub timeout:Option<u32>,
    pub loglevel:String,
}

impl Default for Config{
    fn default() -> Self {
        Config{
            llm:LLM::default(),
            loglevel:"INFO".into(),
            timeout:Some(3),
            resource_dir:None,
            template:"./templates/exmaple.template.html".to_string(),
            proxy:None,
        }
    }
}



