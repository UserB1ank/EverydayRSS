
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct Group{
    name:String,
    fees:Vec<Feed>
}

#[derive(Debug, Serialize, Deserialize)]
struct Feed{
    name:String,
    url:String,
    enable:bool,
}

#[cfg(test)]
mod tests{
    use std::env;
    use std::path::PathBuf;

    #[test]
    fn test_parse_toml(){
        let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
        let project_root = PathBuf::from(manifest_dir);

    }
    #[test]
    fn test_get_root_path(){
        let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
        let project_root = PathBuf::from(manifest_dir);
        assert_eq!(project_root.display().to_string(),"F:\\coding\\EverydayRSS");
    }
}