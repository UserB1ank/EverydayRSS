use crate::Args;
use crate::parser::load_resources_from_dir;

pub fn run(args:Args)->Result<(),Box<dyn std::error::Error>>{
    // TODO implement arguments validate

    // TODO is dir existing
    let resources_dir=args.resource_dir;
    let rss_resources=load_resources_from_dir(resources_dir.unwrap())?;
    if rss_resources.iter().count()<=0{
        return Err(format!("resources error : {:?}",rss_resources).into())
    }

    Ok(())
}