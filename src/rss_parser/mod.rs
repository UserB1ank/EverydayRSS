mod toml;

use std::path::PathBuf;
use std::error::Error;
use std::fs;
use crate::rss_parser::toml::parse_toml;

pub fn load_resources_from_dir(resource_dir:PathBuf) ->Result<(),Box<dyn Error>>{
    for entry in fs::read_dir(&resource_dir)?{
        #[cfg(debug_assertions)]
        println!("{:?}",entry);
        let file=entry?;
        if file.metadata()?.is_file(){
            #[cfg(debug_assertions)]
            println!("rss file path: {:?}",file.path());
            // match extension
            match file.path() {
                _=>{
                    return Err(format!("unsupported file type: {:?}",file.path()).into())
                }
            }
        }
    }
    Ok(())
}