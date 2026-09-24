mod app;
mod config;
mod llm;
mod model;
mod notify;
mod onboarding;
mod parser;
mod render;
mod schedule;

use std::io::{self, IsTerminal};
use std::path::PathBuf;

use app::AppError;
use clap::{Parser, Subcommand};
use tracing_subscriber::EnvFilter;

use crate::config::{default_config_path, load_config, save_config};
use crate::model::config::ScheduleConfig;

#[derive(Debug, Parser)]
#[command(
    name = "everydayrss",
    version,
    about = "Fetch recent RSS articles and generate AI summaries"
)]
struct Cli {
    /// Path to the configuration file
    #[arg(long, global = true, env = "EVERYDAYRSS_CONFIG")]
    config: Option<PathBuf>,

    /// Temporarily override the feed directory
    #[arg(short, long, global = true)]
    resource_dir: Option<PathBuf>,

    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Fetch feeds, generate summaries, and render one report
    Run {
        /// Temporarily override the article date window in hours
        #[arg(long, value_parser = clap::value_parser!(u32).range(1..))]
        lookback_hours: Option<u32>,
    },
    /// Start the initial setup wizard
    Init {
        /// Skip overwrite confirmation and rebuild invalid configuration from defaults
        #[arg(long)]
        force: bool,
    },
    /// Install, inspect, or remove the scheduled task
    Schedule {
        #[command(subcommand)]
        command: ScheduleCommand,
    },
}

#[derive(Debug, Subcommand)]
enum ScheduleCommand {
    /// Register the scheduled task
    Install {
        /// Hour of day to run (0-23)
        #[arg(long, conflicts_with = "every_hours", value_parser = clap::value_parser!(u8).range(0..=23))]
        hour: Option<u8>,
        /// Minute of hour to run (0-59)
        #[arg(long, conflicts_with = "every_hours", value_parser = clap::value_parser!(u8).range(0..=59))]
        minute: Option<u8>,
        /// Run every N hours instead
        #[arg(long, conflicts_with_all = ["hour", "minute"], value_parser = clap::value_parser!(u32).range(1..))]
        every_hours: Option<u32>,
    },
    /// Show scheduled task status
    Status,
    /// Remove the scheduled task
    Remove,
}

#[tokio::main]
async fn main() -> Result<(), AppError> {
    let cli = Cli::parse();
    let config_path = absolute_path(cli.config.unwrap_or(default_config_path()?))?;

    match cli.command {
        Some(Command::Init { force }) => {
            initialize(&config_path, force)?;
        }
        Some(Command::Schedule { command }) => handle_schedule(command, &config_path)?,
        Some(Command::Run { lookback_hours }) => {
            run_once(cli.resource_dir, lookback_hours, config_path).await?
        }
        None if !config_path.exists() && io::stdin().is_terminal() => {
            println!("EverydayRSS is not configured yet. Starting the setup wizard.");
            initialize(&config_path, false)?;
            println!("Setup complete. Run `everydayrss run` to generate your first report.");
        }
        None => run_once(cli.resource_dir, None, config_path).await?,
    }
    Ok(())
}

fn initialize(config_path: &std::path::Path, force: bool) -> Result<(), AppError> {
    let outcome = onboarding::run(config_path, force)?;
    if let Some(schedule) = outcome.schedule {
        let path = install_schedule(config_path, schedule.clone())?;
        println!(
            "✓ Scheduled task registered ({}): {}",
            schedule.description(),
            path.display()
        );
    }
    Ok(())
}

async fn run_once(
    resource_dir: Option<PathBuf>,
    lookback_hours: Option<u32>,
    config_path: PathBuf,
) -> Result<(), AppError> {
    let config = load_config(&config_path)?;
    init_tracing(&config.loglevel)?;
    let output = app::run(resource_dir, lookback_hours, config_path, config).await?;
    println!("✓ Report generated: {}", output.display());
    Ok(())
}

fn handle_schedule(
    command: ScheduleCommand,
    config_path: &std::path::Path,
) -> Result<(), AppError> {
    match command {
        ScheduleCommand::Install {
            hour,
            minute,
            every_hours,
        } => {
            let schedule = every_hours.map_or_else(
                || ScheduleConfig::Daily {
                    hour: hour.unwrap_or(8),
                    minute: minute.unwrap_or(0),
                },
                |hours| ScheduleConfig::Interval { hours },
            );
            let path = install_schedule(config_path, schedule.clone())?;
            println!(
                "✓ Scheduled task registered ({}): {}",
                schedule.description(),
                path.display()
            );
        }
        ScheduleCommand::Status => {
            let status = schedule::status()?;
            println!("Definition file: {}", status.definition_path.display());
            println!("Installed: {}", yes_no(status.installed));
            println!("Loaded: {}", yes_no(status.loaded));
            if let Ok(config) = load_config(config_path)
                && let Some(schedule) = config.schedule
            {
                println!("Configured schedule: {}", schedule.description());
            }
        }
        ScheduleCommand::Remove => {
            let path = schedule::remove()?;
            if let Ok(mut config) = load_config(config_path) {
                config.schedule = None;
                save_config(config_path, &config)?;
            }
            println!("✓ Scheduled task removed: {}", path.display());
        }
    }
    Ok(())
}

fn install_schedule(
    config_path: &std::path::Path,
    schedule_config: ScheduleConfig,
) -> Result<PathBuf, AppError> {
    let mut config = load_config(config_path)?;
    let path = schedule::install(config_path, &schedule_config)?;
    config.schedule = Some(schedule_config);
    save_config(config_path, &config)?;
    Ok(path)
}

fn init_tracing(level: &str) -> Result<(), AppError> {
    let level = level.to_ascii_lowercase();
    let filter = EnvFilter::try_new(format!("warn,everydayrss={level}"))?;
    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .try_init()?;
    Ok(())
}

fn absolute_path(path: PathBuf) -> Result<PathBuf, AppError> {
    if path.is_absolute() {
        Ok(path)
    } else {
        Ok(std::env::current_dir()?.join(path))
    }
}

fn yes_no(value: bool) -> &'static str {
    if value { "yes" } else { "no" }
}
