use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TomlRss {
    pub group:Vec<GroupRss>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GroupRss {
    pub name:String,
    pub feed:Vec<FeedResource>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeedResource {
    pub name:String,
    pub url:String,
    pub enabled:bool,
}