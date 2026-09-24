use std::collections::{BTreeMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::time::Duration;

use reqwest::{Client, Proxy};
use tracing::{debug, error, info, warn};

use crate::config::resolve_path;
use crate::model::article::Feed;
use crate::model::config::Config;
use crate::model::summary::FeedSummary;
use crate::notify;
use crate::parser::load_resources_from_dir;
use crate::parser::rss_parser::{DateFilter, fetch};
use crate::render::render_html;

pub async fn run(
    resource_dir_override: Option<PathBuf>,
    lookback_hours_override: Option<u32>,
    config_path: PathBuf,
    cfg: Config,
) -> Result<PathBuf, AppError> {
    validate_config(&cfg)?;

    let resources_dir = if let Some(path) = resource_dir_override {
        path
    } else {
        cfg.resource_dir
            .as_deref()
            .map(|path| resolve_path(&config_path, path))
            .unwrap_or_else(|| {
                config_path
                    .parent()
                    .unwrap_or_else(|| std::path::Path::new("."))
                    .join("resources")
            })
    };
    let template_path = resolve_path(&config_path, &cfg.template);
    let output_path = resolve_path(&config_path, &cfg.output);
    validate_paths(&resources_dir, &template_path)?;

    let c_args = ClientArgs {
        timeout: cfg.timeout,
        proxy: cfg.proxy.clone(),
    };
    let feed_resources = load_resources_from_dir(resources_dir)?;
    if feed_resources.is_empty() {
        return Err("No usable TOML files were found in the feed directory".into());
    }

    let llm = &cfg.llm;

    let filter = DateFilter {
        lookback_hours: resolve_lookback_hours(&cfg, lookback_hours_override),
        include_undated: cfg.include_undated,
    };
    let mut summary_groups: BTreeMap<String, Vec<FeedSummary>> = BTreeMap::new();
    let mut seen_urls = HashSet::new();
    for toml_rss in feed_resources {
        for group in toml_rss.group {
            let mut summaries = vec![];
            info!(
                "Group {}, total count feeds {} ",
                group.name,
                group.feed.len()
            );
            let mut tasks = Vec::new();
            let req_client = build_client(&c_args)?;
            for feed in group.feed {
                if !feed.enabled {
                    debug!("Skip disabled feed {} {}", feed.name, feed.url);
                    continue;
                }
                if !seen_urls.insert(feed.url.clone()) {
                    warn!("Skip duplicated feed URL {}", feed.url);
                    continue;
                }

                info!("Fetching articles from {}", feed.name);
                let cli = req_client.clone();
                let task = tokio::spawn(async move { fetch(&feed, cli, filter).await });
                tasks.push(task);
            }
            let mut feeds: Vec<Feed> = vec![];
            for task in tasks {
                match task.await {
                    Ok(Ok(result)) => {
                        if result.articles.is_empty() {
                            info!("No recent articles in {}, skipping", result.title);
                        } else {
                            debug!("Task finished successfully, rss content : {:?}", result);
                            feeds.push(result);
                        }
                    }
                    Ok(Err(e)) => {
                        warn!("Task finished with error: {}", e);
                    }
                    Err(e) => {
                        error!("Task failed with : {}", e);
                    }
                }
            }
            for feed in feeds {
                info!("Getting summary of {}", feed.title);
                let cli = req_client.clone();
                let res = llm.get_summary(cli, &feed).await?;
                debug!("Got summary: {:?}", res);
                summaries.push(res);
            }
            summary_groups
                .entry(group.name)
                .or_default()
                .extend(summaries);
        }
    }

    let output = render_html(summary_groups, &template_path)?;
    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(&output_path, &output)?;
    info!("Report written to {}", output_path.display());
    let notification_client = build_client(&c_args)?;
    notify::dispatch(
        &cfg.notifications,
        &output_path,
        &output,
        notification_client,
    )
    .await
    .map_err(|error| {
        format!(
            "The report was generated at {}, but delivery failed: {error}",
            output_path.display()
        )
    })?;
    Ok(output_path)
}

pub async fn push(
    file_override: Option<PathBuf>,
    config_path: PathBuf,
    cfg: Config,
) -> Result<PathBuf, AppError> {
    if !notify::has_enabled_channel(&cfg.notifications) {
        return Err(
            "No notification channel is enabled. Run `everydayrss init` to configure one".into(),
        );
    }
    notify::validate(&cfg.notifications)?;

    let report_path = match file_override {
        Some(path) => {
            let path = if path.is_absolute() {
                path
            } else {
                std::env::current_dir()?.join(path)
            };
            if !path.is_file() {
                return Err(format!("Report file not found: {}", path.display()).into());
            }
            path
        }
        None => latest_report(&resolve_path(&config_path, &cfg.output))?,
    };

    let html = fs::read_to_string(&report_path)?;
    let client = build_client(&ClientArgs {
        timeout: cfg.timeout,
        proxy: cfg.proxy.clone(),
    })?;
    notify::dispatch(&cfg.notifications, &report_path, &html, client)
        .await
        .map_err(|error| format!("Failed to push {}: {error}", report_path.display()))?;
    Ok(report_path)
}

/// Pick the most recently modified HTML report next to the configured output.
fn latest_report(output_path: &Path) -> Result<PathBuf, AppError> {
    let mut newest: Option<(std::time::SystemTime, PathBuf)> = None;
    if let Some(dir) = output_path.parent()
        && dir.is_dir()
    {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if !path.is_file() || !is_html(&path) {
                continue;
            }
            let modified = entry.metadata()?.modified()?;
            if newest.as_ref().is_none_or(|(time, _)| modified > *time) {
                newest = Some((modified, path));
            }
        }
    }
    if let Some((_, path)) = newest {
        return Ok(path);
    }
    if output_path.is_file() {
        return Ok(output_path.to_path_buf());
    }
    Err(format!(
        "No HTML report found in {}. Run `everydayrss run` first",
        output_path.display()
    )
    .into())
}

fn is_html(path: &Path) -> bool {
    path.extension()
        .and_then(|extension| extension.to_str())
        .is_some_and(|extension| extension.eq_ignore_ascii_case("html"))
}

pub type AppError = Box<dyn std::error::Error + Send + Sync>;

#[derive(Debug, Clone)]
struct ClientArgs {
    pub timeout: Option<u32>,
    pub proxy: Option<String>,
}

fn build_client(args: &ClientArgs) -> Result<Client, AppError> {
    let timeout = args.timeout.unwrap_or(10);
    let mut builder = Client::builder();
    builder = builder
        .connect_timeout(Duration::from_secs(u64::from(timeout)))
        .read_timeout(Duration::from_secs(30));
    let proxy = args.proxy.clone();
    if let Some(proxy) = proxy {
        let proxy = Proxy::all(proxy)?;
        builder = builder.proxy(proxy);
    }
    builder = builder.user_agent("EverydayRSS");
    Ok(builder.build()?)
}

fn validate_config(config: &Config) -> Result<(), AppError> {
    if config.lookback_hours == Some(0) {
        return Err("lookback_hours must be greater than 0".into());
    }
    if let Some(schedule) = &config.schedule {
        match schedule {
            crate::model::config::ScheduleConfig::Daily { hour, minute }
                if *hour > 23 || *minute > 59 =>
            {
                return Err("The daily schedule time is invalid".into());
            }
            crate::model::config::ScheduleConfig::Interval { hours } if *hours == 0 => {
                return Err("The schedule interval must be greater than 0 hours".into());
            }
            _ => {}
        }
    }
    if config.llm.base_url.trim().is_empty()
        || config.llm.api_key.trim().is_empty()
        || config.llm.model.trim().is_empty()
    {
        return Err(
            "The LLM configuration is incomplete. Run `everydayrss init` to update it".into(),
        );
    }
    if config.llm.summary_language.trim().is_empty() {
        return Err("Summary language cannot be empty".into());
    }
    notify::validate(&config.notifications)?;
    Ok(())
}

fn resolve_lookback_hours(config: &Config, override_hours: Option<u32>) -> u32 {
    override_hours
        .or(config.lookback_hours)
        .or_else(|| {
            config
                .schedule
                .as_ref()
                .map(|schedule| schedule.cycle_hours())
        })
        .unwrap_or(24)
}

fn validate_paths(resources_dir: &Path, template_path: &Path) -> Result<(), AppError> {
    if !resources_dir.is_dir() {
        return Err(format!("Feed directory not found: {}", resources_dir.display()).into());
    }
    if !template_path.is_file() {
        return Err(format!("HTML template not found: {}", template_path.display()).into());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::resolve_lookback_hours;
    use crate::model::config::{Config, ScheduleConfig};

    #[test]
    fn date_window_follows_schedule_unless_overridden() {
        let mut config = Config {
            schedule: Some(ScheduleConfig::Interval { hours: 6 }),
            ..Config::default()
        };
        assert_eq!(resolve_lookback_hours(&config, None), 6);

        config.lookback_hours = Some(12);
        assert_eq!(resolve_lookback_hours(&config, None), 12);
        assert_eq!(resolve_lookback_hours(&config, Some(2)), 2);
    }

    #[test]
    fn date_window_defaults_to_daily_cycle() {
        assert_eq!(resolve_lookback_hours(&Config::default(), None), 24);
    }
}
