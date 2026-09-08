mod toml_parser;
mod rss_parser;

use std::path::PathBuf;
use std::error::Error;
use std::fs;
use crate::parser::toml_parser::{parse_toml, TomlRss};

pub fn load_resources_from_dir(resource_dir:PathBuf) ->Result<Vec<TomlRss>,Box<dyn Error>>{
    let mut rss_resources:Vec<TomlRss>=vec![];
    for entry in fs::read_dir(&resource_dir)?{
        #[cfg(debug_assertions)]
        println!("{:?}",entry);
        let file=entry?;
        if file.metadata()?.is_file(){
            #[cfg(debug_assertions)]
            println!("rss file path: {:?}",file.path());
            // match extension
            match file.path().extension() {
                 Some(ext)=>{
                     if let Some(ext_str)=ext.to_str(){
                         match ext_str.to_lowercase().as_str() {
                             "toml"=>{
                                 rss_resources.push(parse_toml(&file.path())?)
                             },
                             _ => {
                                 // return Err(format!("unsupported file type: {:?}",file.path()).into())
                                 eprintln!("unsupported file type: {:?}",file.path())

                             }
                         }
                     }
                }
                None=>{
                    // return Err(format!("no file extension: {:?}",file.path()).into())
                    eprintln!("no file extension: {:?}",file.path())
                }
            }
        }
    }
    #[cfg(debug_assertions)]
    dbg!(&rss_resources);

    Ok(rss_resources)
}

#[cfg(test)]
mod tests{

}