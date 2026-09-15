use serde::{Deserialize, Serialize};

/// result of fetch feed
#[derive(Debug,Clone,Default,Serialize)]
pub struct Feed{
    pub title:String,
    pub url:String,
    pub articles:Vec<Article>
}

/// description: summary content description
/// title: tictle
/// url: link
#[derive(Debug,Default,Clone,Serialize)]
pub struct Article {
    pub title:String,
    pub url:String,
    pub description:String
}
