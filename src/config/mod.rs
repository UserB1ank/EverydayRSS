use std::fs;
use std::path::PathBuf;
use tracing::debug;
use crate::app::AppError;
use crate::config::example::EXAMPLE_FEED_TOML;
use crate::model::config::Config;
pub mod prompts;
pub mod example;

pub fn get_config_dir() ->Result<PathBuf,AppError>{
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
        debug!("Created config file at {}", config_path.display());
        // init resources dir
        let resource_dir=config_dir.join("resources");
        fs::create_dir_all(&resource_dir)?;
        debug!("Created resources directory at {}", resource_dir.display());
        let example_toml_path=resource_dir.join("feeds.example.toml");
        fs::write(&example_toml_path, EXAMPLE_FEED_TOML)?;
        debug!("Created example.toml at {}", example_toml_path.display());
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