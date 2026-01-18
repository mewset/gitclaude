use anyhow::Result;

use crate::config::NotifyConfig;

/// Send a desktop notification
pub fn send_notification(title: &str, body: &str, config: &NotifyConfig) -> Result<()> {
    let urgency = match config.urgency.as_str() {
        "low" => notify_rust::Urgency::Low,
        "critical" => notify_rust::Urgency::Critical,
        _ => notify_rust::Urgency::Normal,
    };

    notify_rust::Notification::new()
        .summary(title)
        .body(body)
        .appname(&config.app_name)
        .timeout(notify_rust::Timeout::Milliseconds(config.timeout))
        .urgency(urgency)
        .show()?;

    Ok(())
}

/// Send a notification with the Claude response
pub fn notify_response(response: &str, event: &str, config: &NotifyConfig) -> Result<()> {
    let title = format!("gitclaude - {}", event);

    // Truncate long responses for notification
    let body = if response.len() > 200 {
        format!("{}...", &response[..200])
    } else {
        response.to_string()
    };

    send_notification(&title, &body, config)
}
