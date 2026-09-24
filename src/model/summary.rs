use crate::model::article::Article;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct FeedSummary {
    conclusion: String,
    posts: Vec<Post>,
}
#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct Post {
    section: String,
    articles: Vec<Article>,
    #[serde(default)]
    analysis: String,
}

#[cfg(test)]
mod tests {
    use super::FeedSummary;

    #[test]
    fn accepts_llm_output_without_analysis() {
        let json = r#"{
            "conclusion": "Daily summary",
            "posts": [{
                "section": "Security",
                "articles": [{
                    "title": "Example",
                    "url": "https://example.com/article",
                    "description": "Example summary"
                }]
            }]
        }"#;

        let result = serde_json::from_str::<FeedSummary>(json);
        assert!(
            result.is_ok(),
            "LLM output without analysis should remain usable: {}",
            result.unwrap_err()
        );
    }
}
