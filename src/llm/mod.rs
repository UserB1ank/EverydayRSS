use crate::config::prompts::PROMPTS;
use crate::model::article::Feed;
use crate::model::summary::FeedSummary;

pub fn get_summary(articles:&Feed) ->Result<FeedSummary,Box<dyn std::error::Error>>{
    let feed_summary:FeedSummary=FeedSummary::default();
    
    Ok(feed_summary)
}