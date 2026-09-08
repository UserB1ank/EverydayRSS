use std::fs;
use std::path::PathBuf;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TomlRss {
    group:Vec<Group>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Group{
    name:String,
    feed:Vec<Feed>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Feed{
    name:String,
    url:String,
    enabled:bool,
}


pub fn parse_toml(rss_path: &PathBuf) -> Result<TomlRss, Box<dyn std::error::Error>>{
    let content=fs::read_to_string(rss_path)?;
    let toml_content: TomlRss =toml::from_str(&content)?;
    Ok(toml_content)
}

#[cfg(test)]
mod tests{
    use std::env;
    use std::path::PathBuf;
    use crate::parser::toml_parser::parse_toml;

    #[test]
    fn test_parse_toml(){
        let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
        let project_root = PathBuf::from(manifest_dir);
        let resource_dir=project_root.join("resources").join("feeds.example.toml");
        let group=parse_toml(&resource_dir).unwrap();
        dbg!(group);
    }
    #[test]
    fn test_get_root_path(){
        let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
        let project_root = PathBuf::from(manifest_dir);
        dbg!(project_root);
        // assert_eq!(project_root.display().to_string(),"EverydayRSS");

    }
}