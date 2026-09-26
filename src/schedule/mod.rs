use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use crate::app::AppError;
use crate::model::config::ScheduleConfig;

#[cfg(target_os = "macos")]
const LABEL: &str = "com.everydayrss.daily";

#[derive(Debug)]
pub struct ScheduleStatus {
    pub definition_path: PathBuf,
    pub installed: bool,
    pub loaded: bool,
}

pub fn install(config_path: &Path, schedule: &ScheduleConfig) -> Result<PathBuf, AppError> {
    validate_schedule(schedule)?;
    let executable = std::env::current_exe()?;

    #[cfg(target_os = "macos")]
    {
        install_launchd(config_path, &executable, schedule)
    }
    #[cfg(target_os = "linux")]
    {
        install_systemd(config_path, &executable, schedule)
    }
    #[cfg(not(any(target_os = "macos", target_os = "linux")))]
    {
        let _ = (config_path, executable);
        Err("Automatic scheduled task registration is not supported on this platform".into())
    }
}

pub fn remove() -> Result<PathBuf, AppError> {
    #[cfg(target_os = "macos")]
    {
        remove_launchd()
    }
    #[cfg(target_os = "linux")]
    {
        remove_systemd()
    }
    #[cfg(not(any(target_os = "macos", target_os = "linux")))]
    {
        Err("Automatic scheduled task removal is not supported on this platform".into())
    }
}

pub fn status() -> Result<ScheduleStatus, AppError> {
    #[cfg(target_os = "macos")]
    {
        status_launchd()
    }
    #[cfg(target_os = "linux")]
    {
        status_systemd()
    }
    #[cfg(not(any(target_os = "macos", target_os = "linux")))]
    {
        Err("Scheduled task status is not supported on this platform".into())
    }
}

fn validate_schedule(schedule: &ScheduleConfig) -> Result<(), AppError> {
    match schedule {
        ScheduleConfig::Daily { hour, minute } if *hour > 23 || *minute > 59 => {
            Err("The scheduled time must be between 00:00 and 23:59".into())
        }
        ScheduleConfig::Interval { hours } if *hours == 0 => {
            Err("The schedule interval must be greater than 0 hours".into())
        }
        _ => Ok(()),
    }
}

#[cfg(target_os = "macos")]
fn launchd_path() -> Result<PathBuf, AppError> {
    let home = dirs::home_dir().ok_or("Unable to determine the user home directory")?;
    Ok(home
        .join("Library")
        .join("LaunchAgents")
        .join(format!("{LABEL}.plist")))
}

#[cfg(target_os = "macos")]
fn launchd_domain() -> Result<String, AppError> {
    let output = run(Command::new("id").arg("-u"), "Read the current user ID")?;
    let uid = String::from_utf8(output.stdout)?.trim().to_string();
    Ok(format!("gui/{uid}"))
}

#[cfg(target_os = "macos")]
fn install_launchd(
    config_path: &Path,
    executable: &Path,
    schedule: &ScheduleConfig,
) -> Result<PathBuf, AppError> {
    let path = launchd_path()?;
    let domain = launchd_domain()?;
    let config_dir = config_path.parent().unwrap_or_else(|| Path::new("."));
    let logs_dir = config_dir.join("logs");
    fs::create_dir_all(&logs_dir)?;
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    // Unload an older definition before replacing it. A missing job is harmless.
    let _ = Command::new("launchctl")
        .args(["bootout", &domain])
        .arg(&path)
        .output();

    let definition = render_launchd(
        config_path,
        executable,
        config_dir,
        &logs_dir.join("everydayrss.log"),
        &logs_dir.join("everydayrss.error.log"),
        schedule,
    );
    fs::write(&path, definition)?;

    run(
        Command::new("launchctl")
            .args(["bootstrap", &domain])
            .arg(&path),
        "Load the launchd scheduled task",
    )?;
    Ok(path)
}

#[cfg(target_os = "macos")]
fn remove_launchd() -> Result<PathBuf, AppError> {
    let path = launchd_path()?;
    let domain = launchd_domain()?;
    if path.exists() {
        let _ = Command::new("launchctl")
            .args(["bootout", &domain])
            .arg(&path)
            .output();
        fs::remove_file(&path)?;
    }
    Ok(path)
}

#[cfg(target_os = "macos")]
fn status_launchd() -> Result<ScheduleStatus, AppError> {
    let path = launchd_path()?;
    let domain = launchd_domain()?;
    let loaded = Command::new("launchctl")
        .arg("print")
        .arg(format!("{domain}/{LABEL}"))
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false);
    Ok(ScheduleStatus {
        installed: path.exists(),
        definition_path: path,
        loaded,
    })
}

#[cfg(target_os = "macos")]
fn render_launchd(
    config_path: &Path,
    executable: &Path,
    working_dir: &Path,
    stdout_path: &Path,
    stderr_path: &Path,
    schedule: &ScheduleConfig,
) -> String {
    let trigger = match schedule {
        ScheduleConfig::Daily { hour, minute } => format!(
            "  <key>StartCalendarInterval</key>\n  <dict><key>Hour</key><integer>{hour}</integer><key>Minute</key><integer>{minute}</integer></dict>"
        ),
        ScheduleConfig::Interval { hours } => format!(
            "  <key>StartInterval</key><integer>{}</integer>\n  <key>RunAtLoad</key><true/>",
            u64::from(*hours) * 60 * 60
        ),
    };
    format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
  <key>Label</key><string>{LABEL}</string>
  <key>ProgramArguments</key>
  <array>
    <string>{}</string>
    <string>--config</string>
    <string>{}</string>
    <string>run</string>
  </array>
  <key>WorkingDirectory</key><string>{}</string>
{trigger}
  <key>StandardOutPath</key><string>{}</string>
  <key>StandardErrorPath</key><string>{}</string>
  <key>ProcessType</key><string>Background</string>
</dict>
</plist>
"#,
        xml_escape(&executable.display().to_string()),
        xml_escape(&config_path.display().to_string()),
        xml_escape(&working_dir.display().to_string()),
        xml_escape(&stdout_path.display().to_string()),
        xml_escape(&stderr_path.display().to_string()),
    )
}

#[cfg(target_os = "linux")]
fn systemd_dir() -> Result<PathBuf, AppError> {
    let config =
        dirs::config_dir().ok_or("Unable to determine the user configuration directory")?;
    Ok(config.join("systemd").join("user"))
}

#[cfg(target_os = "linux")]
fn install_systemd(
    config_path: &Path,
    executable: &Path,
    schedule: &ScheduleConfig,
) -> Result<PathBuf, AppError> {
    let dir = systemd_dir()?;
    fs::create_dir_all(&dir)?;
    let service_path = dir.join("everydayrss.service");
    let timer_path = dir.join("everydayrss.timer");
    let working_dir = config_path.parent().unwrap_or_else(|| Path::new("."));
    let service = render_systemd_service(config_path, executable, working_dir);
    let trigger = match schedule {
        ScheduleConfig::Daily { hour, minute } => {
            format!("OnCalendar=*-*-* {hour:02}:{minute:02}:00\nPersistent=true")
        }
        ScheduleConfig::Interval { hours } => {
            format!("OnBootSec=5m\nOnUnitActiveSec={hours}h\nPersistent=true")
        }
    };
    let timer = format!(
        "[Unit]\nDescription=Run EverydayRSS on schedule\n\n[Timer]\n{trigger}\nUnit=everydayrss.service\n\n[Install]\nWantedBy=timers.target\n"
    );
    fs::write(&service_path, service)?;
    fs::write(&timer_path, timer)?;
    run(
        Command::new("systemctl").args(["--user", "daemon-reload"]),
        "Reload the systemd user configuration",
    )?;
    run(
        Command::new("systemctl").args(["--user", "enable", "--now", "everydayrss.timer"]),
        "Enable the systemd scheduled task",
    )?;
    ensure_linger();
    Ok(timer_path)
}

#[cfg(target_os = "linux")]
fn remove_systemd() -> Result<PathBuf, AppError> {
    let dir = systemd_dir()?;
    let service_path = dir.join("everydayrss.service");
    let timer_path = dir.join("everydayrss.timer");
    let _ = Command::new("systemctl")
        .args(["--user", "disable", "--now", "everydayrss.timer"])
        .output();
    if timer_path.exists() {
        fs::remove_file(&timer_path)?;
    }
    if service_path.exists() {
        fs::remove_file(service_path)?;
    }
    let _ = Command::new("systemctl")
        .args(["--user", "daemon-reload"])
        .output();
    Ok(timer_path)
}

#[cfg(target_os = "linux")]
fn status_systemd() -> Result<ScheduleStatus, AppError> {
    let path = systemd_dir()?.join("everydayrss.timer");
    let loaded = Command::new("systemctl")
        .args(["--user", "is-active", "--quiet", "everydayrss.timer"])
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false);
    Ok(ScheduleStatus {
        installed: path.exists(),
        definition_path: path,
        loaded,
    })
}

#[cfg(target_os = "linux")]
fn ensure_linger() {
    let username = match Command::new("id").arg("-un").output() {
        Ok(output) if output.status.success() => {
            String::from_utf8_lossy(&output.stdout).trim().to_string()
        }
        _ => return,
    };
    if username.is_empty() {
        return;
    }

    let lingering = Command::new("loginctl")
        .arg("show-user")
        .arg(&username)
        .args(["--property=Linger", "--value"])
        .output()
        .ok()
        .filter(|output| output.status.success())
        .is_some_and(|output| String::from_utf8_lossy(&output.stdout).trim() == "yes");
    if lingering {
        return;
    }

    // Self-linger is permitted for active users on most distributions; when
    // it is not, surface the privileged command instead of failing install.
    let enabled = Command::new("loginctl")
        .arg("enable-linger")
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false);
    if enabled {
        println!(
            "✓ Linger enabled for user {username}: the timer now fires without an active login."
        );
    } else {
        println!(
            "⚠ Linger is off for user {username}: the timer only fires while this user is logged in."
        );
        println!("  Make it persistent with: sudo loginctl enable-linger {username}");
    }
}

#[cfg(target_os = "linux")]
fn systemd_argument(path: &Path) -> String {
    // Older systemd releases treat a leading double quote as part of the
    // executable path, so escape special characters instead of quoting.
    systemd_escape(path, true)
}

#[cfg(target_os = "linux")]
fn systemd_working_directory(path: &Path) -> String {
    // WorkingDirectory= is not shell-like and does not strip surrounding
    // quotes. Escape whitespace instead of quoting the complete path.
    systemd_escape(path, false)
}

#[cfg(target_os = "linux")]
fn systemd_escape(path: &Path, escape_quotes: bool) -> String {
    let mut escaped = String::new();
    for ch in path.display().to_string().chars() {
        match ch {
            '%' => escaped.push_str("%%"),
            '\\' => escaped.push_str("\\\\"),
            '"' if escape_quotes => escaped.push_str("\\\""),
            ' ' => escaped.push_str("\\x20"),
            '\t' => escaped.push_str("\\t"),
            '\n' => escaped.push_str("\\n"),
            '\r' => escaped.push_str("\\r"),
            _ => escaped.push(ch),
        }
    }
    escaped
}

#[cfg(target_os = "linux")]
fn render_systemd_service(config_path: &Path, executable: &Path, working_dir: &Path) -> String {
    format!(
        "[Unit]\nDescription=EverydayRSS daily digest\n\n[Service]\nType=oneshot\nWorkingDirectory={}\nExecStart={} --config {} run\n",
        systemd_working_directory(working_dir),
        systemd_argument(executable),
        systemd_argument(config_path),
    )
}

fn run(command: &mut Command, description: &str) -> Result<Output, AppError> {
    let output = command.output()?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("{description} failed: {}", stderr.trim()).into());
    }
    Ok(output)
}

#[cfg(target_os = "macos")]
fn xml_escape(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

#[cfg(test)]
mod tests {
    use super::validate_schedule;
    use crate::model::config::ScheduleConfig;

    #[test]
    fn schedule_time_is_bounded() {
        assert!(validate_schedule(&ScheduleConfig::Daily { hour: 0, minute: 0 }).is_ok());
        assert!(
            validate_schedule(&ScheduleConfig::Daily {
                hour: 23,
                minute: 59
            })
            .is_ok()
        );
        assert!(
            validate_schedule(&ScheduleConfig::Daily {
                hour: 24,
                minute: 0
            })
            .is_err()
        );
        assert!(validate_schedule(&ScheduleConfig::Interval { hours: 0 }).is_err());
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn launchd_definition_escapes_paths() {
        use std::path::Path;

        let definition = super::render_launchd(
            Path::new("/tmp/a&b/config.toml"),
            Path::new("/tmp/everydayrss"),
            Path::new("/tmp/a&b"),
            Path::new("/tmp/a&b/out.log"),
            Path::new("/tmp/a&b/error.log"),
            &ScheduleConfig::Daily {
                hour: 8,
                minute: 30,
            },
        );
        assert!(definition.contains("/tmp/a&amp;b/config.toml"));
        assert!(definition.contains("<integer>8</integer>"));
        assert!(definition.contains("<integer>30</integer>"));
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn launchd_interval_uses_start_interval() {
        use std::path::Path;

        let definition = super::render_launchd(
            Path::new("/tmp/config.toml"),
            Path::new("/tmp/everydayrss"),
            Path::new("/tmp"),
            Path::new("/tmp/out.log"),
            Path::new("/tmp/error.log"),
            &ScheduleConfig::Interval { hours: 6 },
        );
        assert!(definition.contains("<key>StartInterval</key><integer>21600</integer>"));
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn systemd_service_escapes_paths_without_quotes() {
        use std::path::Path;

        let definition = super::render_systemd_service(
            Path::new("/tmp/rss config/config.toml"),
            Path::new("/usr/local/bin/everydayrss"),
            Path::new("/tmp/rss config"),
        );

        assert!(definition.contains("WorkingDirectory=/tmp/rss\\x20config\n"));
        assert!(definition.contains(
            "ExecStart=/usr/local/bin/everydayrss --config /tmp/rss\\x20config/config.toml run"
        ));
        assert!(!definition.contains('"'));
    }
}
