use reqwest::Client;
use reqwest::header::{ACCEPT, ACCEPT_ENCODING};
use crate::app::AppError;
use crate::model::article::Feed;
use crate::model::config::LLM;
use crate::model::summary::FeedSummary;

pub fn get_summary(_articles:&Feed) ->Result<FeedSummary,AppError>{
    let feed_summary:FeedSummary=FeedSummary::default();
    
    Ok(feed_summary)
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
}