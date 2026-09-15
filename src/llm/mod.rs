use feed_rs::model::FeedType::JSON;
use reqwest::{Body, Client};
use reqwest::header::{ACCEPT, ACCEPT_ENCODING};
use serde::{Deserialize, Serialize};
use crate::app::AppError;
use crate::config::prompts::PROMPTS;
use crate::config::template::JSON_EXAMPLE;
use crate::model::article::Feed;
use crate::model::config::LLM;
use crate::model::summary::FeedSummary;
use reqwest_eventsource::{Event, EventSource};
#[derive(Debug, Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<Message>,
    temperature: f32,
    response_format: ResponseFormat,
    thinking: Thinking,
    reasoning_effort: String,
    stream: bool
}
/// enable or disable
#[derive(Debug, Serialize, Deserialize)]
struct Thinking {
    #[serde(rename = "type")]
    think_type: String,
}
#[derive(Debug, Serialize, Deserialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize,Serialize)]
struct ResponseFormat {
    #[serde(rename="type")]
    format_type:String
}

#[derive(Debug, Deserialize)]
pub struct ChatChunk {
    pub id: Option<String>,
    pub choices: Vec<ChunkChoice>,
}

#[derive(Debug, Deserialize)]
pub struct ChunkChoice {
    pub index: u32,
    pub delta: Delta,
    pub finish_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Delta {
    pub role: Option<String>,
    #[serde(default)]
    pub content: Option<String>,
    // 一些提供商用 `reasoning_content` 传输思考流：
    #[serde(default)]
    pub reasoning_content: Option<String>,
}

impl LLM {
    pub async  fn is_llm_reachable(&self, cli:Client)->Result<bool,AppError>{
        cli
            .get(&self.base_url)
            .header(ACCEPT, "*/*")
            .header(ACCEPT_ENCODING, "identity")
            .send()
            .await?;

        Ok(true)
    }

    pub async fn get_summary(&self,cli:Client,articles:&Feed) ->Result<FeedSummary,AppError>{
        let feed_summary:FeedSummary=FeedSummary::default();
        let body:ChatRequest=ChatRequest{
            model: self.model.clone(),

            response_format:ResponseFormat{
                format_type:"json_object".to_string()
            },
            thinking: Thinking{think_type: "enable".parse().unwrap() },
            reasoning_effort: "medium".to_string(),
            temperature:1.0,
            messages:vec![
                Message{
                    role:"system".to_string(),
                    content:PROMPTS.to_string(),
                },
                Message{
                    role:"user".to_string(),
                    content:serde_json::to_string(articles)?,
                }
            ],
            stream: true,
        };
        let res = cli
            .post(format!(
                "{}/chat/completions",
                self.base_url.trim_end_matches('/')
            ))
            .bearer_auth(&self.api_key)
            .json(&body)
            .send()
            .await?
            .error_for_status()?;

        if !res.status().is_success() {
            let status = res.status();
            let text = res.text().await?;
            return Err(format!("llm request error: {}, {}", status, text).into());
        }
        Ok(feed_summary)
    }
}