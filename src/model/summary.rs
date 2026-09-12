use crate::model::article::Article;

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