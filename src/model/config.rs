pub(crate) use crate::model::llm::Llm;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(default)]
pub struct Config {
    pub llm: Llm,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_dir: Option<String>,
    pub template: String,
    pub output: String,
    pub proxy: Option<String>,
    pub timeout: Option<u32>,
    pub loglevel: String,
    /// Explicit article window. `None` means derive it from the schedule cycle.
    pub lookback_hours: Option<u32>,
    pub include_undated: bool,
    pub schedule: Option<ScheduleConfig>,
    pub notifications: NotificationConfig,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            llm: Llm::default(),
            loglevel: "INFO".into(),
            timeout: Some(10),
            resource_dir: None,
            template: "templates/example.template.html".to_string(),
            output: "output/daily.html".to_string(),
            proxy: None,
            lookback_hours: None,
            include_undated: true,
            schedule: None,
            notifications: NotificationConfig::default(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum ScheduleConfig {
    Daily { hour: u8, minute: u8 },
    Interval { hours: u32 },
}

impl ScheduleConfig {
    pub fn cycle_hours(&self) -> u32 {
        match self {
            Self::Daily { .. } => 24,
            Self::Interval { hours } => *hours,
        }
    }

    pub fn description(&self) -> String {
        match self {
            Self::Daily { hour, minute } => format!("daily at {hour:02}:{minute:02}"),
            Self::Interval { hours } => format!("every {hours} hours"),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct NotificationConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<EmailConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wecom: Option<WeComConfig>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(default)]
pub struct EmailConfig {
    pub enabled: bool,
    pub smtp_host: String,
    pub smtp_port: u16,
    /// `starttls`, `tls`, or `none`.
    pub security: String,
    pub username: String,
    pub password: String,
    pub from: String,
    pub to: Vec<String>,
    pub subject: String,
}

impl Default for EmailConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            smtp_host: String::new(),
            smtp_port: 587,
            security: "starttls".to_string(),
            username: String::new(),
            password: String::new(),
            from: String::new(),
            to: Vec::new(),
            subject: "EverydayRSS Daily Digest".to_string(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct WeComConfig {
    pub enabled: bool,
    pub webhook_url: String,
}
