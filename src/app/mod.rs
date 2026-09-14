use std::path::PathBuf;
use std::str::FromStr;
use EverydayRSS::config::{get_config, get_config_dir};
use EverydayRSS::model::config::Config;
use crate::Args;
use EverydayRSS::parser::load_resources_from_dir;

pub fn run(args:Args)->Result<(),Box<dyn std::error::Error>>{
    // init config
    let cfg_dir=get_config_dir()?;
    let cfg:Config=get_config()?;

    // TODO implement arguments validate
    // validate resource dir,
    // if there's no args, use the location in config toml
    // else, use default config location
    let resources_dir=if let Some(r)=args.resource_dir{
        r
    }else {
        match cfg.resource_dir {
            Some(r) => {
                PathBuf::from_str(&r)?
            },
            None=>cfg_dir.join("resources"),
        }
    };
    let rss_resources=load_resources_from_dir(resources_dir);
    if rss_resources.iter().count()<=0{
        return Err(format!("resources error : {:?}",rss_resources).into())
    }
    Ok(())
}

#[cfg(test)]
mod tests{
    use std::path::PathBuf;

    
}