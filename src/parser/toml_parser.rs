use crate::app::AppError;
use crate::model::resource::TomlRss;
use std::fs;
use std::path::PathBuf;

pub fn parse_rss_toml(rss_path: &PathBuf) -> Result<TomlRss, AppError> {
    let content = fs::read_to_string(rss_path)?;
    let toml_content: TomlRss = toml::from_str(&content)?;
    Ok(toml_content)
}

#[cfg(test)]
mod tests {
    use crate::app::AppError;
    use crate::parser::toml_parser::parse_rss_toml;
    use std::path::PathBuf;

    #[test]
    fn test_parse_toml() -> Result<(), AppError> {
        let resource = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/feeds.toml");
        let parsed = parse_rss_toml(&resource)?;
        assert_eq!(parsed.group.len(), 1);
        assert_eq!(parsed.group[0].name, "Technology");
        assert_eq!(parsed.group[0].feed[0].url, "https://example.test/feed.xml");
        Ok(())
    }
}
