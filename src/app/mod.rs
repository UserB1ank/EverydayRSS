use crate::Args;
use crate::config::{get_config, get_config_dir};
use crate::model::article::Feed;
use crate::model::config::Config;
use crate::model::resource::GroupRss;
use crate::parser::load_resources_from_dir;
use crate::parser::rss_parser::fetch;
use std::path::PathBuf;
use std::str::FromStr;
use tracing::{debug, info};

pub async fn run(args: Args, cfg: Config) -> Result<(), AppError> {
    // parse resource
    // TODO implement arguments validate
    // validate resource dir,
    // if there's no args, use the location in config toml
    // else, use default config location
    let resources_dir = if let Some(r) = args.resource_dir {
        r
    } else {
        match cfg.resource_dir {
            Some(r) => PathBuf::from_str(&r)?,
            None => {
                let cfg_dir = get_config_dir()?;
                cfg_dir.join("resources")
            }
        }
    };
    let feed_resources = load_resources_from_dir(resources_dir)?;
    if feed_resources.iter().count() <= 0 {
        return Err(format!("resources error : {:?}", feed_resources).into());
    }

    // fetch resource
    let mut tasks=Vec::new();
    let timeout=cfg.timeout;
    for toml_rss in feed_resources {
        for group in toml_rss.group {
            info!(
                "Group {}, total count feeds {} ",
                group.name,
                group.feed.iter().count()
            );
            for feed in group.feed{
                let proxy=cfg.proxy.clone();
                if feed.enabled!=true{
                    debug!("Skip disabled feed {} {}",feed.name,feed.url);
                    continue;
                }
                info!("Fetching articles from {}",feed.name);
                let task=tokio::spawn(async move {
                    fetch(&feed.url,&timeout,&proxy).await
                });
                tasks.push(task);
            }
        }
    }
    // let mut feeds: Vec<Feed> = vec![];
    // for task in tasks{
    //     match task.await{
    //         Ok(Ok(result))=>{
    //
    //         }
    //     }
    // }

    // LLM analyse

    // Render HTML

    // notify
    Ok(())
}

pub type AppError = Box<dyn std::error::Error + Send + Sync>;
#[cfg(test)]
mod tests {}
