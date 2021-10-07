use std::time::{Duration, Instant};

use json::object;

const ALERT_THRESHOLD: Duration = Duration::from_secs(5 * 60); // 5 mins

pub fn maybe_send(
    open_since: Instant,
    notified_at: Option<Instant>,
    webhook: &str,
) -> Option<Instant> {
    let now = Instant::now();
    let open_for = now.duration_since(open_since);
    if open_for > ALERT_THRESHOLD && notified_at.is_none() {
        let open_for_time = if open_for.as_secs() > 60 {
            let minutes = open_for.as_secs() / 60;
            format!("{} minute{}", minutes, plural(minutes))
        } else {
            format!(
                "{} second{}",
                open_for.as_secs(),
                plural(open_for.as_secs())
            )
        };
        let mut message = String::from("Garage door has been open for ");
        message.push_str(&open_for_time);
        if let Err(err) = post_webhook(&message, webhook) {
            eprintln!("Error posting notification: {}", err);
        }
        Some(now)
    } else {
        None
    }
}

fn post_webhook(message: &str, webhook: &str) -> Result<(), ureq::Error> {
    let body = object! {
        text: message
    };

    let _resp = ureq::post(webhook)
        .set("Content-Type", "application/json")
        .send_string(&json::stringify(body))?;

    Ok(())
}

fn plural(value: u64) -> &'static str {
    if value == 1 {
        ""
    } else {
        "s"
    }
}
