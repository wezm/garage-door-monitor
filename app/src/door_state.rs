use std::fmt;

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

impl fmt::Display for DoorState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DoorState::Open => f.write_str("Open"),
            DoorState::Closed => f.write_str("Closed"),
            DoorState::Unknown => f.write_str("Unknown"),
        }
    }
}
