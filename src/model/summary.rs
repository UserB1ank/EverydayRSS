use crate::model::article::Article;

pub struct GroupSummary{
    name:String,
    feed_summaries:Vec<FeedSummary>
}

#[derive(Debug,Default)]
pub struct FeedSummary{
    conclusion:String,
    posts:Vec<Post>
}
#[derive(Debug,Default)]
pub struct Post{
    section:String,
    articles:Vec<Article>
}