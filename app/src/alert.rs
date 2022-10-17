use std::time::{Duration, Instant};

use json::object;
use log::error;

use crate::Timestamp;

const ALERT_THRESHOLD: Duration = Duration::from_secs(5 * 60); // 5 mins

pub fn maybe_send(
    timestamp: Timestamp,
    notified_at: Option<Instant>,
    webhook: &str,
) -> Option<Instant> {
    let now = Instant::now();
    match timestamp {
        Timestamp::None => None,
        Timestamp::OpenSince(open_since) => {
            let open_for = now.duration_since(open_since);
            if open_for > ALERT_THRESHOLD && notified_at.is_none() {
                let open_for_time = open_for_time(open_for);
                let mut message = String::from("Garage door has been open for ");
                message.push_str(&open_for_time);
                if let Err(err) = post_webhook(&message, webhook) {
                    error!("Error posting notification: {}", err);
                }
                Some(now)
            } else {
                None
            }
        }
        Timestamp::ClosedAfter(duration) => {
            if duration > ALERT_THRESHOLD && notified_at.is_none() {
                let open_for_time = open_for_time(duration);
                let mut message = String::from("Garage door closed after ");
                message.push_str(&open_for_time);
                message.push_str(" open");
                if let Err(err) = post_webhook(&message, webhook) {
                    error!("Error posting notification: {}", err);
                }
                Some(now)
            } else {
                None
            }
        }
    }
}

fn open_for_time(open_for: Duration) -> String {
    if open_for.as_secs() > 60 {
        let minutes = open_for.as_secs() / 60;
        format!("{} minute{}", minutes, plural(minutes))
    } else {
        format!(
            "{} second{}",
            open_for.as_secs(),
            plural(open_for.as_secs())
        )
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
