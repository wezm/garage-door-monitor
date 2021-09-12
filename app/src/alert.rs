use std::time::{Duration, Instant};

const ALERT_THRESHOLD: Duration = Duration::from_secs(5 * 60); // 5 mins

pub fn maybe_send(open_since: Instant, notified_at: &mut Option<Instant>) {
    let now = Instant::now();
    let open_for = now.duration_since(open_since);
    if open_for > ALERT_THRESHOLD {}
}
