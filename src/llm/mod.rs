use crate::app::AppError;
use crate::config::prompts::PROMPTS;
use crate::model::article::Feed;
use crate::model::config::Llm;
use crate::model::summary::FeedSummary;
use eventsource_stream::Eventsource;
use futures_util::StreamExt;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tracing::{debug, info};

#[derive(Debug, Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<Message>,
    temperature: f32,
    response_format: ResponseFormat,
    stream: bool,
}
#[derive(Debug, Serialize, Deserialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct ResponseFormat {
    #[serde(rename = "type")]
    format_type: String,
}

impl Llm {
    pub async fn get_summary(&self, cli: Client, articles: &Feed) -> Result<FeedSummary, AppError> {
        let body: ChatRequest = ChatRequest {
            model: self.model.clone(),

            response_format: ResponseFormat {
                format_type: "json_object".to_string(),
            },
            temperature: 0.2,
            messages: vec![
                Message {
                    role: "system".to_string(),
                    content: PROMPTS.to_string(),
                },
                Message {
                    role: "user".to_string(),
                    content: serde_json::to_string(articles)?,
                },
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
            .await?;

        if !res.status().is_success() {
            let status = res.status();
            let text = res.text().await?;
            return Err(format!("llm request error: {}, {}", status, text).into());
        }

        let mut stream = res.bytes_stream().eventsource();

        let mut response: String = "".to_string();
        let mut reasoning: String = "".to_string();
        while let Some(event) = stream.next().await {
            let event = event?;
            if event.data == "[DONE]" {
                break;
            }
            let v: Value = serde_json::from_str(&event.data)?;
            if let Some(text) = v["choices"][0]["delta"]["content"].as_str() {
                response.push_str(text);
            }
            if let Some(text) = v["choices"][0]["delta"]["reasoning_content"].as_str() {
                reasoning.push_str(text);
            }
        }
        info!("Got summary {} bytes", response.len());
        debug!(
            "Got reason {} bytes, content {}",
            reasoning.len(),
            reasoning
        );
        debug!("Got summary {} bytes, content {}", response.len(), response);
        // parse feed summary
        let data: FeedSummary = serde_json::from_str(response.as_str())?;

        Ok(data)
    }
}

#[cfg(test)]
mod tests {
    use crate::config::example::EXAMPLE_SUMMARY_OUTPUT;
    use crate::model::summary::FeedSummary;

    #[test]
    fn test_json_parse() {
        let result = serde_json::from_str::<FeedSummary>(EXAMPLE_SUMMARY_OUTPUT);
        assert!(result.is_ok(), "Parsing failed: {}", result.unwrap_err());
    }
}
