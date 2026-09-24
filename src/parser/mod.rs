pub mod rss_parser;
pub mod toml_parser;

use crate::app::AppError;
use crate::model::resource::TomlRss;
use crate::parser::toml_parser::parse_rss_toml;
use std::fs;
use std::path::PathBuf;
use tracing::{debug, error, info};

pub fn load_resources_from_dir(resource_dir: PathBuf) -> Result<Vec<TomlRss>, AppError> {
    let mut rss_resources: Vec<TomlRss> = vec![];
    debug!("Load resource from {:?}", resource_dir);
    let mut entries = fs::read_dir(&resource_dir)?.collect::<Result<Vec<_>, _>>()?;
    entries.sort_by_key(|entry| entry.path());
    for file in entries {
        debug!("Load entry {:?}, resource {:?}", file, resource_dir);
        if file.metadata()?.is_file() {
            info!("Load rss, file path: {:?}", file.path());
            // match extension
            match file.path().extension() {
                Some(ext) => {
                    if let Some(ext_str) = ext.to_str() {
                        match ext_str.to_lowercase().as_str() {
                            "toml" => rss_resources.push(parse_rss_toml(&file.path())?),
                            _ => {
                                // return Err(format!("unsupported file type: {:?}",file.path()).into())
                                error!("unsupported file type: {:?}", file.path())
                            }
                        }
                    }
                }
                None => {
                    // return Err(format!("no file extension: {:?}",file.path()).into())
                    error!("no file extension: {:?}", file.path())
                }
            }
        }
    }
    debug!("Result of Load Resource {:?}", rss_resources);

    Ok(rss_resources)
}
