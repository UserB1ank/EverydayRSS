use std::fs;
use std::path::PathBuf;

pub mod prompts;

pub mod db;

pub fn get_config_dir()->Result<PathBuf,Box<dyn std::error::Error>>{
    let home=dirs::home_dir();
    match home {
        Some(home)=>{
            let config_dir=home.join(".everydayrss");
            fs::create_dir_all(&config_dir)?;
            Ok(config_dir)
        }
        None=>{
            Err("Can't find home dir".into())
        }
    }
}