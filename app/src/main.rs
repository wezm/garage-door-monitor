use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{mpsc, Arc, RwLock};
use std::time::{Duration, Instant};
use std::{io, process, thread};

use rppal::gpio::{Gpio, InputPin, OutputPin};

use garage_door_monitor::{alert, http, led, DoorState, State};

const DOOR_PIN: u8 = 20; // header pin 38
const LED_PIN: u8 = 21; // header pin 40
const ONE_SECOND: Duration = Duration::from_secs(1);
const SERVER_ADDR: (&str, u16) = ("0.0.0.0", 8888);

fn main() -> Result<(), io::Error> {
    let term = Arc::new(AtomicBool::new(false));
    signal_hook::flag::register(signal_hook::consts::SIGINT, Arc::clone(&term)).map(|_| ())?;
    signal_hook::flag::register(signal_hook::consts::SIGTERM, Arc::clone(&term)).map(|_| ())?;

    let (tx, rx) = mpsc::channel();
    let state = Arc::new(RwLock::new(State {
        door_state: DoorState::Unknown,
        open_since: None,
        notified_at: None,
    }));
    let pins = setup_gpio();
    let mut threads = Vec::new();

    // GPIO thread
    // The GPIO thread is only spawned if we were able to acquire the pins
    // If a physical inspection of the device shows no flashing then this state should be obvious.
    match pins {
        Ok((door, mut led)) => {
            let term = Arc::clone(&term);
            let thread = thread::spawn(move || {
                while !term.load(Ordering::Relaxed) {
                    let door_state = door.read().into();
                    tx.send(door_state).expect("send error"); // TODO: Work out how to handle this best

                    match door_state {
                        DoorState::Closed => led::flash(&mut led, 1),
                        DoorState::Open => led::flash(&mut led, 2),
                        DoorState::Unknown => led::flash(&mut led, 3),
                    }
                    std::thread::sleep(ONE_SECOND);
                }
                eprintln!("GPIO thread exiting");
            });
            threads.push(thread);
        }
        Err(err) => {
            eprintln!("Unable to set up GPIO: {}", err)
        }
    }

    // State management thread
    {
        let term = Arc::clone(&term);
        let state = Arc::clone(&state);
        let thread = thread::spawn(move || {
            while !term.load(Ordering::Relaxed) {
                if let Ok(door_state) = rx.recv_timeout(ONE_SECOND) {
                    let current_state = { *state.read().unwrap() };
                    let new_state = match (door_state, current_state.open_since) {
                        // Closed to open transition
                        (DoorState::Open, None) => State {
                            door_state,
                            open_since: Some(Instant::now()),
                            notified_at: None,
                        },
                        // Open to closed transition
                        (DoorState::Closed, Some(_)) => State {
                            door_state,
                            open_since: None,
                            notified_at: None,
                        },
                        _ => State {
                            door_state,
                            ..current_state
                        },
                    };
                    if new_state != current_state {
                        *state.write().unwrap() = new_state;
                    }
                }
            }
            eprintln!("state management thread exiting");
        });
        threads.push(thread);
    }

    // Notification thread
    {
        let term = Arc::clone(&term);
        let state = Arc::clone(&state);
        let thread = thread::spawn(move || {
            while !term.load(Ordering::Relaxed) {
                // I don't want to hold the lock while the notification is sent. If it's slow
                // then it will block other things from happening, however there could be a
                // read-modify-write case if the state is updated while the notification is
                // sent. Since we clear notified_at when detecting and opening this is ok.
                let current_state = { *state.read().unwrap() };
                let maybe_sent = current_state.open_since.and_then(|open_since| {
                    alert::maybe_send(open_since, current_state.notified_at)
                });
                if maybe_sent.is_some() {
                    // notification was sent, update state
                    let mut current_state = state.write().unwrap();
                    current_state.notified_at = maybe_sent
                }

                std::thread::sleep(5 * ONE_SECOND);
            }
            eprintln!("notification thread exiting");
        });
        threads.push(thread);
    }

    // Start HTTP server
    let server = match http::Server::new(SERVER_ADDR) {
        Ok(server) => Arc::new(server),
        Err(err) => {
            eprintln!(
                "Unable to start http server on {}:{}: {}",
                SERVER_ADDR.0, SERVER_ADDR.1, err
            );
            process::exit(1);
        }
    };
    eprintln!("http server running on {}:{}", SERVER_ADDR.0, SERVER_ADDR.1);

    // HTTP server shutdown thread
    {
        let term = Arc::clone(&term);
        let server = Arc::clone(&server);
        let thread = thread::spawn(move || {
            while !term.load(Ordering::Relaxed) {
                std::thread::sleep(5 * ONE_SECOND);
            }
            server.shutdown();
            eprintln!("server thread exiting");
        });
        threads.push(thread);
    }

    // Handle HTTP requests
    server.handle_requests(state);

    for thread in threads {
        let _ = thread.join();
    }

    Ok(())
}

fn setup_gpio() -> rppal::gpio::Result<(InputPin, OutputPin)> {
    let gpio = Gpio::new()?;
    let door_pin = gpio.get(DOOR_PIN)?.into_input_pulldown();
    let led_pin = gpio.get(LED_PIN)?.into_output();
    Ok((door_pin, led_pin))
}
