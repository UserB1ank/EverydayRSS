use std::fs;
use std::path::PathBuf;
use crate::app::AppError;
use crate::model::config::Config;
use crate::parser::toml_parser::parse_rss_toml;

pub mod prompts;


pub fn get_config_dir()->Result<PathBuf,AppError>{
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
/// Get config from config dir, default location is ~/.everydayrss/config.toml
pub fn get_config()->Result<Config,AppError>{
    let config_dir=get_config_dir()?;
    let config_path=config_dir.join("config.toml");

    if !config_path.exists(){
        let config=Config::default();

        let content=toml::to_string_pretty(&config)?;
        fs::write(&config_path,content)?;
        return Ok(config);
    }

    let r=fs::read_to_string(config_path)?;
    let cfg=toml::from_str(&r)?;
    Ok(cfg)
}

#[cfg(test)]
mod tests{
    use std::path::PathBuf;


}