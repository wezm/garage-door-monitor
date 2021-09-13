pub mod alert;
mod door_state;
pub mod http;
pub mod led;

use std::time::Instant;

pub use door_state::{AtomicDoorState, DoorState};

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct State {
    pub door_state: DoorState,
    pub open_since: Option<Instant>,
    pub notified_at: Option<Instant>,
}
