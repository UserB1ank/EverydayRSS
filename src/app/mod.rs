use crate::Args;
use crate::rss_parser;
use crate::rss_parser::load_resources_from_dir;

pub fn run(args:Args)->Result<(),Box<dyn std::error::Error>>{
    // TODO implement arguments validate

    // TODO is dir existing
    let resources_dir=args.resource_dir;
    load_resources_from_dir(resources_dir.unwrap())?;
    Ok(())
}