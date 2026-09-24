use std::path::Path;

use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor};
use reqwest::multipart::{Form, Part};
use reqwest::{Client, Url};
use serde::Deserialize;
use serde_json::json;

use crate::app::AppError;
use crate::model::config::{EmailConfig, NotificationConfig, WeComConfig};

pub fn has_enabled_channel(config: &NotificationConfig) -> bool {
    config.email.as_ref().is_some_and(|email| email.enabled)
        || config.wecom.as_ref().is_some_and(|wecom| wecom.enabled)
}

pub fn validate(config: &NotificationConfig) -> Result<(), AppError> {
    if let Some(email) = config.email.as_ref().filter(|email| email.enabled) {
        validate_email(email)?;
    }
    if let Some(wecom) = config.wecom.as_ref().filter(|wecom| wecom.enabled) {
        let webhook = Url::parse(&wecom.webhook_url).map_err(|_| "Invalid WeCom webhook URL")?;
        derive_upload_url(&webhook)?;
    }
    Ok(())
}

pub async fn dispatch(
    config: &NotificationConfig,
    report_path: &Path,
    html: &str,
    client: Client,
) -> Result<(), AppError> {
    let mut failures = Vec::new();

    if let Some(email) = config.email.as_ref().filter(|email| email.enabled)
        && let Err(error) = send_email(email, html).await
    {
        failures.push(format!("Email: {error}"));
    }

    if let Some(wecom) = config.wecom.as_ref().filter(|wecom| wecom.enabled)
        && let Err(error) = send_wecom_file(wecom, report_path, html, client).await
    {
        failures.push(format!("WeCom: {error}"));
    }

    if failures.is_empty() {
        Ok(())
    } else {
        Err(failures.join("; ").into())
    }
}

async fn send_email(config: &EmailConfig, html: &str) -> Result<(), AppError> {
    validate_email(config)?;

    let mut builder = Message::builder()
        .from(config.from.parse()?)
        .subject(&config.subject);
    for recipient in &config.to {
        builder = builder.to(recipient.parse()?);
    }
    let message = builder
        .header(ContentType::TEXT_HTML)
        .body(html.to_string())?;

    let mut transport = match config.security.to_ascii_lowercase().as_str() {
        "starttls" => AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(&config.smtp_host)?,
        "tls" => AsyncSmtpTransport::<Tokio1Executor>::relay(&config.smtp_host)?,
        "none" => AsyncSmtpTransport::<Tokio1Executor>::builder_dangerous(&config.smtp_host),
        _ => return Err("Email security must be starttls, tls, or none".into()),
    }
    .port(config.smtp_port);

    if !config.username.is_empty() {
        transport = transport.credentials(Credentials::new(
            config.username.clone(),
            config.password.clone(),
        ));
    }

    transport.build().send(message).await?;
    Ok(())
}

fn validate_email(config: &EmailConfig) -> Result<(), AppError> {
    if config.smtp_host.trim().is_empty() {
        return Err("SMTP host cannot be empty".into());
    }
    if config.smtp_port == 0 {
        return Err("SMTP port must be greater than 0".into());
    }
    if config.from.trim().is_empty() || config.to.is_empty() {
        return Err("A sender and at least one recipient are required".into());
    }
    let _: lettre::message::Mailbox = config.from.parse()?;
    for recipient in &config.to {
        let _: lettre::message::Mailbox = recipient.parse()?;
    }
    match config.security.to_ascii_lowercase().as_str() {
        "starttls" | "tls" | "none" => {}
        _ => return Err("Email security must be starttls, tls, or none".into()),
    }
    Ok(())
}

async fn send_wecom_file(
    config: &WeComConfig,
    report_path: &Path,
    html: &str,
    client: Client,
) -> Result<(), AppError> {
    const MAX_FILE_SIZE: usize = 20 * 1024 * 1024;
    if html.len() < 5 || html.len() > MAX_FILE_SIZE {
        return Err("The HTML file must be between 5 bytes and 20 MB".into());
    }

    let webhook_url = Url::parse(&config.webhook_url).map_err(|_| "Invalid WeCom webhook URL")?;
    let upload_url = derive_upload_url(&webhook_url)?;
    let filename = report_path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("everydayrss.html")
        .to_string();
    let part = Part::bytes(html.as_bytes().to_vec())
        .file_name(filename)
        .mime_str("text/html")?;
    let upload_response = client
        .post(upload_url)
        .multipart(Form::new().part("media", part))
        .send()
        .await
        .map_err(|error| error.without_url())?
        .error_for_status()
        .map_err(|error| error.without_url())?
        .json::<WeComResponse>()
        .await
        .map_err(|error| error.without_url())?;
    upload_response.ensure_success("Upload file")?;
    let media_id = upload_response
        .media_id
        .ok_or("The WeCom upload response did not include media_id")?;

    let send_response = client
        .post(webhook_url)
        .json(&json!({
            "msgtype": "file",
            "file": { "media_id": media_id }
        }))
        .send()
        .await
        .map_err(|error| error.without_url())?
        .error_for_status()
        .map_err(|error| error.without_url())?
        .json::<WeComResponse>()
        .await
        .map_err(|error| error.without_url())?;
    send_response.ensure_success("Send file")?;
    Ok(())
}

fn derive_upload_url(webhook_url: &Url) -> Result<Url, AppError> {
    let key = webhook_url
        .query_pairs()
        .find_map(|(name, value)| (name == "key").then(|| value.into_owned()))
        .ok_or("The WeCom webhook URL is missing the key parameter")?;
    let mut upload_url = webhook_url.clone();
    upload_url.set_path("/cgi-bin/webhook/upload_media");
    upload_url.set_query(None);
    upload_url
        .query_pairs_mut()
        .append_pair("key", &key)
        .append_pair("type", "file");
    Ok(upload_url)
}

#[derive(Debug, Deserialize)]
struct WeComResponse {
    errcode: i64,
    errmsg: String,
    #[serde(default)]
    media_id: Option<String>,
}

impl WeComResponse {
    fn ensure_success(&self, action: &str) -> Result<(), AppError> {
        if self.errcode == 0 {
            Ok(())
        } else {
            Err(format!("{action} failed ({}): {}", self.errcode, self.errmsg).into())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::derive_upload_url;
    use reqwest::Url;

    #[test]
    fn derives_wecom_upload_url_without_leaking_other_query_values() {
        let webhook =
            Url::parse("https://qyapi.weixin.qq.com/cgi-bin/webhook/send?key=secret&ignored=value")
                .expect("valid URL");
        let upload = derive_upload_url(&webhook).expect("upload URL");

        assert_eq!(
            upload.as_str(),
            "https://qyapi.weixin.qq.com/cgi-bin/webhook/upload_media?key=secret&type=file"
        );
    }

    #[test]
    fn rejects_webhook_without_key() {
        let webhook =
            Url::parse("https://qyapi.weixin.qq.com/cgi-bin/webhook/send").expect("valid URL");
        assert!(derive_upload_url(&webhook).is_err());
    }
}
