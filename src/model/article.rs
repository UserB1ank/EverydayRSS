/// result of fetch feed
#[derive(Debug,Clone)]
pub struct Feed{
    pub title:String,
    pub url:String,
    pub articles:Vec<Article>
}

/// description: summary content description
/// title: tictle
/// url: link
#[derive(Debug,Default,Clone)]
pub struct Article {
    pub title:String,
    pub url:String,
    pub description:String
}
