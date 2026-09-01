mod rss_parser;
mod app;

use clap::Parser;
use std::path::PathBuf;
use serde::de::Error;

#[derive(Parser)]
struct Args {
    // Directory containing RSS resource files
    #[arg(short,long)]
    #[arg()]
    resource_dir: Option<PathBuf>,
}

fn main() ->Result<(),Box<dyn std::error::Error>>{
    let args=Args::parse();
    app::run(args)
}
