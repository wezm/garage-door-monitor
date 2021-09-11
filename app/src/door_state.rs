use std::fmt;
use std::sync::atomic::{AtomicU8, Ordering};
use std::sync::Arc;

use rppal::gpio::Level;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum DoorState {
    Open = 0,
    Closed = 1,
    Unknown = 2,
}

impl From<u8> for DoorState {
    fn from(val: u8) -> Self {
        use DoorState::*;
        match val {
            0 => Open,
            1 => Closed,
            2 => Unknown,
            _ => unreachable!(),
        }
    }
}

impl From<Level> for DoorState {
    fn from(level: Level) -> Self {
        match level {
            Level::Low => DoorState::Open,
            Level::High => DoorState::Closed,
        }
    }
}

#[derive(Clone)]
pub struct AtomicDoorState {
    flag: Arc<AtomicU8>,
}

impl AtomicDoorState {
    pub fn new(state: DoorState) -> Self {
        AtomicDoorState {
            flag: Arc::new(AtomicU8::new(state as u8)),
        }
    }

    pub fn open(&self) {
        self.set_state(DoorState::Open)
    }

    pub fn closed(&self) {
        self.set_state(DoorState::Closed)
    }

    pub fn unknown(&self) {
        self.set_state(DoorState::Unknown)
    }

    #[inline]
    pub fn get_state(&self) -> DoorState {
        self.flag.load(Ordering::SeqCst).into()
    }

    pub fn set_state(&self, state: DoorState) {
        self.flag.store(state as u8, Ordering::SeqCst)
    }
}

impl fmt::Display for DoorState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DoorState::Open => f.write_str("Open"),
            DoorState::Closed => f.write_str("Closed"),
            DoorState::Unknown => f.write_str("Unknown"),
        }
    }
}
