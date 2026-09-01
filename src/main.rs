mod rss_parser;
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
struct Args {
    // Directory containing RSS resource files
    #[arg(short,long)]
    #[arg()]
    resource_dir: Option<PathBuf>,
}

fn main() {
    let args=Args::parse();
    let resource_dir=args.resource_dir;

}
