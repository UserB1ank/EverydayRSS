use serde::{Deserialize, Serialize};
use crate::model::article::Article;


#[derive(Debug,Default,Deserialize,Serialize,Clone)]
pub struct FeedSummary{
    conclusion:String,
    posts:Vec<Post>,
}
#[derive(Debug,Default,Deserialize,Serialize,Clone)]
pub struct Post{
    section:String,
    articles:Vec<Article>,
    analysis:String,
}