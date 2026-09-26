use std::io::{self, IsTerminal};
use std::path::Path;

use dialoguer::{Confirm, Input, Password, Select, theme::ColorfulTheme};

use crate::app::AppError;
use crate::config::{ensure_layout, load_config, save_config};
use crate::model::config::{Config, EmailConfig, ScheduleConfig, WeComConfig};

pub struct InitOutcome {
    pub schedule: Option<ScheduleConfig>,
}

pub fn run(config_path: &Path, force: bool) -> Result<InitOutcome, AppError> {
    if !io::stdin().is_terminal() {
        return Err("The setup wizard requires an interactive terminal. Run `everydayrss init` in a terminal".into());
    }

    let theme = ColorfulTheme::default();
    println!("\nEverydayRSS Setup Wizard\n");

    let exists = config_path.exists();
    if exists
        && !force
        && !Confirm::with_theme(&theme)
            .with_prompt(format!(
                "Configuration {} already exists. Update it?",
                config_path.display()
            ))
            .default(true)
            .interact()?
    {
        return Ok(InitOutcome { schedule: None });
    }

    let mut config = if exists {
        match load_config(config_path) {
            Ok(config) => config,
            Err(error) if force => {
                eprintln!("Unable to read the configuration; rebuilding from defaults: {error}");
                Config::default()
            }
            Err(error) => return Err(error),
        }
    } else {
        Config::default()
    };

    config.llm.provider = text_input(
        &theme,
        "LLM provider label",
        fallback(&config.llm.provider, "openai-compatible"),
    )?;
    config.llm.base_url = text_input(
        &theme,
        "OpenAI-compatible API URL",
        fallback(&config.llm.base_url, "https://api.openai.com/v1"),
    )?;
    config.llm.model = if config.llm.model.trim().is_empty() {
        required_input(&theme, "Model name")?
    } else {
        text_input(&theme, "Model name", &config.llm.model)?
    };
    config.llm.summary_language = text_input(
        &theme,
        "Summary language",
        fallback(&config.llm.summary_language, "Chinese"),
    )?;

    let has_key = !config.llm.api_key.is_empty();
    let api_key = Password::with_theme(&theme)
        .with_prompt(if has_key {
            "API key (leave blank to keep the current value)"
        } else {
            "API Key"
        })
        .allow_empty_password(has_key)
        .interact()?;
    if !api_key.is_empty() {
        config.llm.api_key = api_key;
    }

    let resource_default = config.resource_dir.as_deref().unwrap_or("resources");
    config.resource_dir = Some(text_input(&theme, "Feed directory", resource_default)?);
    config.template = text_input(&theme, "HTML template path", &config.template)?;
    config.output = text_input(&theme, "Report output path", &config.output)?;
    let date_mode = Select::with_theme(&theme)
        .with_prompt("Article date window")
        .items([
            "Follow the scheduled task interval",
            "Custom number of hours",
        ])
        .default(usize::from(config.lookback_hours.is_some()))
        .interact()?;
    config.lookback_hours = if date_mode == 0 {
        None
    } else {
        Some(
            Input::with_theme(&theme)
                .with_prompt("Include articles from the most recent N hours")
                .default(config.lookback_hours.unwrap_or(24))
                .validate_with(|value: &u32| -> Result<(), &str> {
                    if *value > 0 {
                        Ok(())
                    } else {
                        Err("Must be greater than 0")
                    }
                })
                .interact_text()?,
        )
    };
    config.include_undated = Confirm::with_theme(&theme)
        .with_prompt("Include articles without a publication date")
        .default(config.include_undated)
        .interact()?;

    let proxy_default = config.proxy.as_deref().unwrap_or("");
    let proxy: String = Input::with_theme(&theme)
        .with_prompt("HTTP/HTTPS proxy (optional)")
        .default(proxy_default.to_string())
        .allow_empty(true)
        .interact_text()?;
    config.proxy = non_empty(proxy);
    config.timeout = Some(
        Input::with_theme(&theme)
            .with_prompt("Connection timeout in seconds")
            .default(config.timeout.unwrap_or(10))
            .validate_with(|value: &u32| -> Result<(), &str> {
                if *value > 0 {
                    Ok(())
                } else {
                    Err("Must be greater than 0")
                }
            })
            .interact_text()?,
    );

    let mut email = config.notifications.email.take().unwrap_or_default();
    email.enabled = Confirm::with_theme(&theme)
        .with_prompt("Enable SMTP email delivery")
        .default(email.enabled)
        .interact()?;
    if email.enabled {
        email = configure_email(&theme, email)?;
    }
    config.notifications.email = Some(email);

    let mut wecom = config.notifications.wecom.take().unwrap_or_default();
    wecom.enabled = Confirm::with_theme(&theme)
        .with_prompt("Enable WeCom group bot delivery")
        .default(wecom.enabled)
        .interact()?;
    if wecom.enabled {
        wecom = configure_wecom(&theme, wecom)?;
    }
    config.notifications.wecom = Some(wecom);

    ensure_layout(config_path, &config)?;
    save_config(config_path, &config)?;
    println!("\n✓ Configuration saved to {}", config_path.display());

    let internal_scheduler = std::env::var("EVERYDAYRSS_SCHEDULER")
        .is_ok_and(|value| value.eq_ignore_ascii_case("internal"));
    let schedule_prompt = if internal_scheduler {
        "Configure the built-in scheduler now"
    } else {
        "Register the scheduled task now"
    };
    let schedule = if Confirm::with_theme(&theme)
        .with_prompt(schedule_prompt)
        .default(true)
        .interact()?
    {
        Some(configure_schedule(&theme, config.schedule.as_ref())?)
    } else {
        None
    };

    Ok(InitOutcome { schedule })
}

fn configure_email(theme: &ColorfulTheme, mut email: EmailConfig) -> Result<EmailConfig, AppError> {
    email.smtp_host = if email.smtp_host.is_empty() {
        required_input(theme, "SMTP host")?
    } else {
        text_input(theme, "SMTP host", &email.smtp_host)?
    };
    let previous_security = email.security.to_ascii_lowercase();
    let securities = [
        "STARTTLS (commonly port 587)",
        "TLS (commonly port 465)",
        "No encryption (commonly port 25)",
    ];
    let security_default = match previous_security.as_str() {
        "tls" => 1,
        "none" => 2,
        _ => 0,
    };
    email.security = match Select::with_theme(theme)
        .with_prompt("SMTP connection security")
        .items(securities)
        .default(security_default)
        .interact()?
    {
        1 => "tls",
        2 => "none",
        _ => "starttls",
    }
    .to_string();
    let recommended_port = recommended_smtp_port(&email.security);
    let default_port = smtp_port_default(&previous_security, &email.security, email.smtp_port);
    email.smtp_port = Input::with_theme(theme)
        .with_prompt(format!(
            "SMTP port (recommended for {}: {})",
            smtp_security_label(&email.security),
            recommended_port
        ))
        .default(default_port)
        .validate_with(|value: &u16| -> Result<(), &str> {
            if *value > 0 {
                Ok(())
            } else {
                Err("Must be greater than 0")
            }
        })
        .interact_text()?;
    if email.smtp_port != recommended_port {
        eprintln!(
            "Warning: {} commonly uses port {}, but port {} was selected. Keep this only if your SMTP provider requires a custom port.",
            smtp_security_label(&email.security),
            recommended_port,
            email.smtp_port
        );
    }
    email.username = optional_input(theme, "SMTP username (optional)", &email.username)?;
    let password = Password::with_theme(theme)
        .with_prompt(if email.password.is_empty() {
            "SMTP password (optional when authentication is not required)"
        } else {
            "SMTP password (leave blank to keep the current value)"
        })
        .allow_empty_password(true)
        .interact()?;
    if !password.is_empty() {
        email.password = password;
    }

    let from_default = if email.from.is_empty() {
        email.username.as_str()
    } else {
        email.from.as_str()
    };
    email.from = if from_default.is_empty() {
        required_input(theme, "Sender address")?
    } else {
        text_input(theme, "Sender address", from_default)?
    };
    let recipients_default = email.to.join(",");
    let recipients = if recipients_default.is_empty() {
        required_input(theme, "Recipient addresses (comma-separated)")?
    } else {
        text_input(
            theme,
            "Recipient addresses (comma-separated)",
            &recipients_default,
        )?
    };
    email.to = recipients
        .split(',')
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_string)
        .collect();
    email.subject = text_input(theme, "Email subject", &email.subject)?;
    Ok(email)
}

fn recommended_smtp_port(security: &str) -> u16 {
    match security {
        "tls" => 465,
        "none" => 25,
        _ => 587,
    }
}

fn smtp_security_label(security: &str) -> &'static str {
    match security {
        "tls" => "TLS",
        "none" => "unencrypted SMTP",
        _ => "STARTTLS",
    }
}

fn smtp_port_default(previous_security: &str, selected_security: &str, existing_port: u16) -> u16 {
    if previous_security == selected_security && existing_port > 0 {
        existing_port
    } else {
        recommended_smtp_port(selected_security)
    }
}

fn configure_wecom(theme: &ColorfulTheme, mut wecom: WeComConfig) -> Result<WeComConfig, AppError> {
    let webhook = Password::with_theme(theme)
        .with_prompt(if wecom.webhook_url.is_empty() {
            "WeCom group bot webhook URL"
        } else {
            "WeCom group bot webhook URL (leave blank to keep the current value)"
        })
        .allow_empty_password(!wecom.webhook_url.is_empty())
        .interact()?;
    if !webhook.is_empty() {
        wecom.webhook_url = webhook;
    }
    Ok(wecom)
}

fn configure_schedule(
    theme: &ColorfulTheme,
    existing: Option<&ScheduleConfig>,
) -> Result<ScheduleConfig, AppError> {
    let default_mode = usize::from(matches!(existing, Some(ScheduleConfig::Interval { .. })));
    let mode = Select::with_theme(theme)
        .with_prompt("Scheduled task frequency")
        .items(["Once daily at a fixed time", "Every N hours"])
        .default(default_mode)
        .interact()?;
    if mode == 0 {
        let default_time = match existing {
            Some(ScheduleConfig::Daily { hour, minute }) => format!("{hour:02}:{minute:02}"),
            _ => "08:00".to_string(),
        };
        let time: String = Input::with_theme(theme)
            .with_prompt("Daily run time (HH:MM)")
            .default(default_time)
            .validate_with(|value: &String| -> Result<(), &str> {
                parse_time(value)
                    .map(|_| ())
                    .ok_or("Enter a time between 00:00 and 23:59")
            })
            .interact_text()?;
        let (hour, minute) = parse_time(&time).expect("validated schedule time");
        Ok(ScheduleConfig::Daily { hour, minute })
    } else {
        let default_hours = match existing {
            Some(ScheduleConfig::Interval { hours }) => *hours,
            _ => 6,
        };
        let hours = Input::with_theme(theme)
            .with_prompt("Run interval in hours")
            .default(default_hours)
            .validate_with(|value: &u32| -> Result<(), &str> {
                if *value > 0 {
                    Ok(())
                } else {
                    Err("Must be greater than 0")
                }
            })
            .interact_text()?;
        Ok(ScheduleConfig::Interval { hours })
    }
}

fn text_input(
    theme: &ColorfulTheme,
    prompt: &str,
    default: &str,
) -> Result<String, dialoguer::Error> {
    Input::with_theme(theme)
        .with_prompt(prompt)
        .default(default.to_string())
        .validate_with(|value: &String| -> Result<(), &str> {
            if value.trim().is_empty() {
                Err("Cannot be empty")
            } else {
                Ok(())
            }
        })
        .interact_text()
}

fn required_input(theme: &ColorfulTheme, prompt: &str) -> Result<String, dialoguer::Error> {
    Input::with_theme(theme)
        .with_prompt(prompt)
        .validate_with(|value: &String| -> Result<(), &str> {
            if value.trim().is_empty() {
                Err("Cannot be empty")
            } else {
                Ok(())
            }
        })
        .interact_text()
}

fn optional_input(
    theme: &ColorfulTheme,
    prompt: &str,
    default: &str,
) -> Result<String, dialoguer::Error> {
    Input::with_theme(theme)
        .with_prompt(prompt)
        .default(default.to_string())
        .allow_empty(true)
        .interact_text()
}

fn fallback<'a>(value: &'a str, default: &'a str) -> &'a str {
    if value.trim().is_empty() {
        default
    } else {
        value
    }
}

fn non_empty(value: String) -> Option<String> {
    let value = value.trim();
    (!value.is_empty()).then(|| value.to_string())
}

fn parse_time(value: &str) -> Option<(u8, u8)> {
    let (hour, minute) = value.trim().split_once(':')?;
    let hour = hour.parse::<u8>().ok()?;
    let minute = minute.parse::<u8>().ok()?;
    (hour < 24 && minute < 60).then_some((hour, minute))
}

#[cfg(test)]
mod tests {
    use super::{parse_time, recommended_smtp_port, smtp_port_default};

    #[test]
    fn validates_schedule_times() {
        assert_eq!(parse_time("08:30"), Some((8, 30)));
        assert_eq!(parse_time("23:59"), Some((23, 59)));
        assert_eq!(parse_time("24:00"), None);
        assert_eq!(parse_time("10:60"), None);
        assert_eq!(parse_time("ten"), None);
    }

    #[test]
    fn recommends_smtp_ports_for_each_security_mode() {
        assert_eq!(recommended_smtp_port("starttls"), 587);
        assert_eq!(recommended_smtp_port("tls"), 465);
        assert_eq!(recommended_smtp_port("none"), 25);
    }

    #[test]
    fn changes_default_port_when_security_mode_changes() {
        assert_eq!(smtp_port_default("starttls", "tls", 587), 465);
        assert_eq!(smtp_port_default("tls", "starttls", 465), 587);
        assert_eq!(smtp_port_default("starttls", "starttls", 2525), 2525);
    }
}
