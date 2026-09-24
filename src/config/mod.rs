use std::fs;
use std::path::{Path, PathBuf};

use crate::app::AppError;
use crate::config::example::{EXAMPLE_FEED_TOML, EXAMPLE_SUMMARY_TEMPLATE};
use crate::model::config::Config;

pub mod example;
pub mod prompts;

pub fn get_config_dir() -> Result<PathBuf, AppError> {
    dirs::home_dir()
        .map(|home| home.join(".everydayrss"))
        .ok_or_else(|| "Unable to determine the user home directory".into())
}

pub fn default_config_path() -> Result<PathBuf, AppError> {
    Ok(get_config_dir()?.join("config.toml"))
}

pub fn load_config(path: &Path) -> Result<Config, AppError> {
    if !path.exists() {
        return Err(format!(
            "Configuration file not found: {}. Run `everydayrss init` first.",
            path.display()
        )
        .into());
    }

    let raw = fs::read_to_string(path)?;
    Ok(toml::from_str(&raw)?)
}

pub fn save_config(path: &Path, config: &Config) -> Result<(), AppError> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, toml::to_string_pretty(config)?)?;
    secure_config_file(path)?;
    Ok(())
}

/// Create the resource/template/output layout without overwriting edited assets.
pub fn ensure_layout(config_path: &Path, config: &Config) -> Result<(), AppError> {
    let resource_dir = config
        .resource_dir
        .as_deref()
        .map(|path| resolve_path(config_path, path))
        .unwrap_or_else(|| config_base_dir(config_path).join("resources"));
    fs::create_dir_all(&resource_dir)?;
    write_if_missing(&resource_dir.join("feeds.example.toml"), EXAMPLE_FEED_TOML)?;

    let template_path = resolve_path(config_path, &config.template);
    if let Some(parent) = template_path.parent() {
        fs::create_dir_all(parent)?;
    }
    write_if_missing(&template_path, EXAMPLE_SUMMARY_TEMPLATE)?;

    let output_path = resolve_path(config_path, &config.output);
    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent)?;
    }
    Ok(())
}

pub fn resolve_path(config_path: &Path, value: &str) -> PathBuf {
    let expanded = expand_tilde(value);
    if expanded.is_absolute() {
        expanded
    } else {
        config_base_dir(config_path).join(expanded)
    }
}

fn config_base_dir(config_path: &Path) -> PathBuf {
    config_path
        .parent()
        .filter(|path| !path.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."))
        .to_path_buf()
}

fn expand_tilde(value: &str) -> PathBuf {
    if value == "~" {
        return dirs::home_dir().unwrap_or_else(|| PathBuf::from(value));
    }
    if let Some(rest) = value.strip_prefix("~/")
        && let Some(home) = dirs::home_dir()
    {
        return home.join(rest);
    }
    PathBuf::from(value)
}

fn write_if_missing(path: &Path, content: &str) -> Result<(), AppError> {
    if !path.exists() {
        fs::write(path, content)?;
    }
    Ok(())
}

#[cfg(unix)]
fn secure_config_file(path: &Path) -> Result<(), AppError> {
    use std::os::unix::fs::PermissionsExt;

    fs::set_permissions(path, fs::Permissions::from_mode(0o600))?;
    Ok(())
}

#[cfg(not(unix))]
fn secure_config_file(_path: &Path) -> Result<(), AppError> {
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_dir() -> PathBuf {
        let suffix = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock should be after epoch")
            .as_nanos();
        std::env::temp_dir().join(format!("everydayrss-config-{suffix}"))
    }

    #[test]
    fn layout_uses_paths_relative_to_config() -> Result<(), AppError> {
        let dir = temp_dir();
        let config_path = dir.join("config.toml");
        let config = Config::default();

        ensure_layout(&config_path, &config)?;

        assert!(dir.join("resources/feeds.example.toml").exists());
        assert!(dir.join("templates/example.template.html").exists());
        assert!(dir.join("output").is_dir());
        fs::remove_dir_all(dir)?;
        Ok(())
    }

    #[test]
    fn new_fields_have_defaults_when_loading_old_config() -> Result<(), AppError> {
        let config: Config = toml::from_str(
            r#"
template = "template.html"
loglevel = "INFO"

[llm]
provider = "openai"
base_url = "https://example.test/v1"
api_key = "secret"
model = "example"
"#,
        )?;

        assert_eq!(config.lookback_hours, None);
        assert!(config.include_undated);
        assert_eq!(config.output, "output/daily.html");
        assert_eq!(config.llm.summary_language, "Chinese");
        Ok(())
    }

    #[test]
    fn schedule_and_notifications_round_trip() -> Result<(), AppError> {
        use crate::model::config::{EmailConfig, ScheduleConfig};

        let mut config = Config {
            schedule: Some(ScheduleConfig::Interval { hours: 6 }),
            ..Config::default()
        };
        config.notifications.email = Some(EmailConfig {
            enabled: true,
            smtp_host: "smtp.example.test".to_string(),
            from: "rss@example.test".to_string(),
            to: vec!["reader@example.test".to_string()],
            ..EmailConfig::default()
        });

        let encoded = toml::to_string_pretty(&config)?;
        let decoded: Config = toml::from_str(&encoded)?;

        assert_eq!(decoded.schedule, config.schedule);
        assert_eq!(
            decoded.notifications.email.expect("email config").smtp_host,
            "smtp.example.test"
        );
        Ok(())
    }
}
