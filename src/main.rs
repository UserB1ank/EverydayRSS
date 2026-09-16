mod app;
mod config;
mod llm;
mod model;
mod notify;
mod parser;
mod render;

use app::AppError;
use clap::Parser;
use std::path::PathBuf;
use tracing::{debug, error, info};
use tracing_subscriber::EnvFilter;
use crate::config::get_config;
use crate::model::config::Config;

#[derive(Parser)]
pub struct Args {
    // Directory containing RSS resource files
    #[arg(short,long)]
    resource_dir: Option<PathBuf>,
}

#[tokio::main]
async fn main() -> Result<(), AppError> {
    let mut cfg=match get_config() {
        Ok(cfg) => cfg,
        Err(e)=>{
            println!("Config error \n {}",e);
            return Err(e);
        }
    };
    cfg.loglevel=cfg.loglevel.to_lowercase();
    let filter = EnvFilter::new(format!(
        "off,EverydayRSS={}",
        cfg.loglevel
    ));
    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .init()
    ;

    let args = Args::parse();
    // init config

    let cfg:Config=get_config()?;
    match app::run(args,cfg).await {
        Ok(s) => {
            Ok(())
        }
        Err(e) => {
            error!("{}",e);
            Err(e)
        }
    }
}
