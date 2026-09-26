use std::path::PathBuf;
use std::time::Duration;

use chrono::{DateTime, Datelike, Local, LocalResult, TimeZone};
use tokio::time::sleep;
use tracing::{error, info};

use crate::app::{self, AppError};
use crate::config::load_config;
use crate::model::config::ScheduleConfig;

pub async fn run(config_path: PathBuf, run_on_start: bool) -> Result<(), AppError> {
    let mut run_now = run_on_start;

    loop {
        let config = load_config(&config_path)?;
        let schedule = config.schedule.clone().ok_or(
            "No schedule is configured. Run `everydayrss init` or `everydayrss schedule install` first",
        )?;
        validate_schedule(&schedule)?;

        if !run_now {
            let now = Local::now();
            let delay = next_delay(&schedule, now)?;
            let next = Local::now() + chrono::Duration::from_std(delay)?;
            println!(
                "Next run: {} ({})",
                next.format("%Y-%m-%d %H:%M:%S %:z"),
                schedule.description()
            );
            if wait_or_shutdown(delay).await? {
                println!("Scheduler stopped.");
                return Ok(());
            }
        }
        run_now = false;

        // Reload before every execution so notification, feed, and prompt edits
        // are picked up without restarting the container.
        let config = load_config(&config_path)?;
        info!("Running scheduled report generation");
        match app::run(None, None, config_path.clone(), config).await {
            Ok(output) => println!("✓ Scheduled report generated: {}", output.display()),
            Err(error) => {
                error!("Scheduled run failed: {error}");
                eprintln!("Scheduled run failed: {error}");
            }
        }
    }
}

fn validate_schedule(schedule: &ScheduleConfig) -> Result<(), AppError> {
    match schedule {
        ScheduleConfig::Daily { hour, minute } if *hour > 23 || *minute > 59 => {
            Err("The daily schedule time is invalid".into())
        }
        ScheduleConfig::Interval { hours } if *hours == 0 => {
            Err("The schedule interval must be greater than 0 hours".into())
        }
        _ => Ok(()),
    }
}

fn next_delay<Tz>(schedule: &ScheduleConfig, now: DateTime<Tz>) -> Result<Duration, AppError>
where
    Tz: TimeZone + Clone,
{
    match schedule {
        ScheduleConfig::Interval { hours } => Ok(Duration::from_secs(
            u64::from(*hours)
                .checked_mul(60 * 60)
                .ok_or("The schedule interval is too large")?,
        )),
        ScheduleConfig::Daily { hour, minute } => {
            let next = next_daily_run(now.clone(), *hour, *minute)?;
            Ok(next.signed_duration_since(now).to_std()?)
        }
    }
}

fn next_daily_run<Tz>(now: DateTime<Tz>, hour: u8, minute: u8) -> Result<DateTime<Tz>, AppError>
where
    Tz: TimeZone + Clone,
{
    let timezone = now.timezone();
    let mut date = now.date_naive();

    // Seven attempts also cover a daylight-saving transition where a local
    // wall-clock time does not exist on one particular day.
    for _ in 0..7 {
        match timezone.with_ymd_and_hms(
            date.year(),
            date.month(),
            date.day(),
            u32::from(hour),
            u32::from(minute),
            0,
        ) {
            LocalResult::Single(candidate) if candidate > now => return Ok(candidate),
            LocalResult::Ambiguous(first, second) => {
                if first > now && second > now {
                    return Ok(std::cmp::min(first, second));
                }
                if first > now {
                    return Ok(first);
                }
                if second > now {
                    return Ok(second);
                }
            }
            _ => {}
        }
        date = date
            .succ_opt()
            .ok_or("Unable to calculate the next daily run")?;
    }

    Err("Unable to calculate the next daily run in the configured timezone".into())
}

#[cfg(unix)]
async fn wait_or_shutdown(delay: Duration) -> Result<bool, AppError> {
    use tokio::signal::unix::{SignalKind, signal};

    let mut terminate = signal(SignalKind::terminate())?;
    tokio::select! {
        _ = sleep(delay) => Ok(false),
        _ = tokio::signal::ctrl_c() => Ok(true),
        _ = terminate.recv() => Ok(true),
    }
}

#[cfg(not(unix))]
async fn wait_or_shutdown(delay: Duration) -> Result<bool, AppError> {
    tokio::select! {
        _ = sleep(delay) => Ok(false),
        _ = tokio::signal::ctrl_c() => Ok(true),
    }
}

#[cfg(test)]
mod tests {
    use chrono::{FixedOffset, TimeZone};

    use super::next_delay;
    use crate::model::config::ScheduleConfig;

    #[test]
    fn interval_waits_for_the_configured_cycle() {
        let now = FixedOffset::east_opt(8 * 60 * 60)
            .unwrap()
            .with_ymd_and_hms(2026, 9, 26, 9, 0, 0)
            .unwrap();
        let delay = next_delay(&ScheduleConfig::Interval { hours: 6 }, now).unwrap();

        assert_eq!(delay.as_secs(), 6 * 60 * 60);
    }

    #[test]
    fn daily_schedule_uses_the_next_local_wall_clock_time() {
        let timezone = FixedOffset::east_opt(8 * 60 * 60).unwrap();
        let before = timezone.with_ymd_and_hms(2026, 9, 26, 7, 30, 0).unwrap();
        let after = timezone.with_ymd_and_hms(2026, 9, 26, 8, 30, 0).unwrap();
        let schedule = ScheduleConfig::Daily { hour: 8, minute: 0 };

        assert_eq!(next_delay(&schedule, before).unwrap().as_secs(), 30 * 60);
        assert_eq!(
            next_delay(&schedule, after).unwrap().as_secs(),
            23 * 60 * 60 + 30 * 60
        );
    }
}
