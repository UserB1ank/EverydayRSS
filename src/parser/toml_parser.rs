use std::fs;
use std::path::PathBuf;
use crate::app::AppError;
use crate::model::resource::TomlRss;

pub fn parse_rss_toml(rss_path: &PathBuf) -> Result<TomlRss, AppError>{
    let content=fs::read_to_string(rss_path)?;
    let toml_content: TomlRss =toml::from_str(&content)?;
    Ok(toml_content)
}

#[cfg(test)]
mod tests{
    use std::env;
    use std::path::PathBuf;
    use crate::app::AppError;
    use crate::config::get_config_dir;
    use crate::parser::toml_parser::parse_rss_toml;

    #[test]
    fn test_parse_toml()->Result<(),AppError>{
        let cfg_dir=get_config_dir()?;
        let resource=cfg_dir.join("resources").join("feeds.example.toml");
        let group= parse_rss_toml(&resource)?;
        Ok(())
    }
    #[test]
    fn test_get_root_path(){
        let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
        let project_root = PathBuf::from(manifest_dir);
        dbg!(project_root);
        // assert_eq!(project_root.display().to_string(),"EverydayRSS");

    }
}