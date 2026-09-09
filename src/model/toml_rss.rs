use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TomlRss {
    pub group:Vec<Group>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Group{
    pub name:String,
    pub feed:Vec<Feed>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Feed{
    pub name:String,
    pub url:String,
    pub enabled:bool,
}