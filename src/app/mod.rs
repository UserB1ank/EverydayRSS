use crate::Args;
use crate::config::get_config_dir;
use crate::model::article::Feed;
use crate::model::config::{Config, LLM};
use crate::parser::load_resources_from_dir;
use crate::parser::rss_parser::fetch;
use std::path::PathBuf;
use std::str::FromStr;
use std::time::Duration;
use reqwest::{Client, Proxy};
use tracing::{debug, error, info, warn};

pub async fn run(args: Args, cfg: Config) -> Result<(), AppError> {
    // TODO implement cfg validate check

    // parse resource
    // TODO implement arguments validate check
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
    let timeout=cfg.timeout.clone();
    let proxy=cfg.proxy.clone();
    //build req client args
    let c_args=ClientArgs{
        timeout,
        proxy
    };


    let feed_resources = load_resources_from_dir(resources_dir)?;
    if feed_resources.iter().count() <= 0 {
        return Err(format!("resources error : {:?}", feed_resources).into());
    }

    // llm reachable testing
    let llm=cfg.llm;
    let cli=build_client(&c_args)?;
    match llm.is_llm_reachable(cli).await {
        Ok(true) => info!("LLM is reachable"),
        Ok(false) => {
            warn!("LLM is not reachable")
        },
        Err(e) => {
            let msg=format!("LLM reachable testing failed with: {:?}", e);
            error!("{}", msg);
            return Err(msg.into());
        },
    }

    // fetch resource
    for toml_rss in feed_resources {
        for group in toml_rss.group {
            info!(
                "Group {}, total count feeds {} ",
                group.name,
                group.feed.iter().count()
            );
            let mut tasks=Vec::new();
            let req_client=build_client(&c_args)?;
            for feed in group.feed{

                if feed.enabled!=true{
                    debug!("Skip disabled feed {} {}",feed.name,feed.url);
                    continue;
                }

                info!("Fetching articles from {}",feed.name);
                let cli=req_client.clone();
                let task=tokio::spawn(async move {
                    fetch(&feed,cli).await
                });
                tasks.push(task);
            }
            let mut feeds: Vec<Feed> = vec![];
            //fetch content
            for task in tasks{
                match task.await{
                    Ok(Ok(result))=>{
                        debug!("Task finished successfully, rss content : {:?}", result);
                        feeds.push(result);
                    },
                    Ok(Err(e))=>{
                        warn!("Task finished with error: {}", e);
                    },
                    Err(e)=>{
                        error!("Task failed with : {}", e);
                    }
                }
            }
            tasks=Vec::new();
            // get summary
            for feed in feeds{
                
            }

        }
    }



    // Render HTML

    // notify
    Ok(())
}

pub type AppError = Box<dyn std::error::Error + Send + Sync>;

#[derive(Debug,Clone)]
struct ClientArgs {
    pub timeout: Option<u32>,
    pub proxy: Option<String>,
}

fn build_client(args:&ClientArgs)->Result<Client,AppError>{
    //initialize client
    let timeout=args.timeout.unwrap_or(3);
    let mut builder =Client::builder();
    builder = builder.timeout(Duration::from_secs(timeout as u64));
    let proxy=args.proxy.clone();
    if let Some(proxy)=proxy {
        let proxy=Proxy::http(proxy)?;
        builder = builder.proxy(proxy);
    };
    builder = builder.user_agent("EverydayRSS");
    Ok(builder.build()?)
}

#[cfg(test)]
mod tests {}

