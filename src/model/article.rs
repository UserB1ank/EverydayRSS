use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// result of fetch feed
#[derive(Debug, Clone, Default, Serialize)]
pub struct Feed {
    pub title: String,
    pub url: String,
    pub articles: Vec<Article>,
}

/// description: summary content description
/// title: title
/// url: link
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Article {
    pub title: String,
    pub url: String,
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub published_at: Option<DateTime<Utc>>,
}
