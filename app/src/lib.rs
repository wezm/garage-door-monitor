pub mod alert;
mod door_state;
pub mod http;
pub mod led;

use std::time::Instant;

pub use door_state::DoorState;

#[macro_export]
macro_rules! term_on_err {
    ($expr:expr, $term:expr) => {
        match $expr {
            std::result::Result::Ok(val) => val,
            std::result::Result::Err(err) => {
                eprintln!("setting term due to error: {}", err);
                $term.store(true, std::sync::atomic::Ordering::SeqCst);
                break;
            }
        }
    };
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct State {
    pub door_state: DoorState,
    pub open_since: Option<Instant>,
    pub notified_at: Option<Instant>,
}
