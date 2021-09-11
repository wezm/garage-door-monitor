use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{mpsc, Arc};
use std::time::{Duration, Instant};
use std::{io, thread};

use json::object;
use rppal::gpio::{Gpio, InputPin, OutputPin};
use tiny_http::{Response, Server};

use garage_door_monitor::{led, AtomicDoorState, DoorState};

const DOOR_PIN: u8 = 20; // header pin 38
const LED_PIN: u8 = 21; // header pin 40
const ONE_SECOND: Duration = Duration::from_secs(1);

fn main() -> Result<(), io::Error> {
    let term = Arc::new(AtomicBool::new(false));
    signal_hook::flag::register(signal_hook::consts::SIGINT, Arc::clone(&term)).map(|_| ())?;
    signal_hook::flag::register(signal_hook::consts::SIGTERM, Arc::clone(&term)).map(|_| ())?;

    let (tx, rx) = mpsc::channel();
    let door_state = AtomicDoorState::new(DoorState::Unknown);
    let pins = setup_gpio();
    let mut threads = Vec::with_capacity(2);

    // GPIO thread
    // The GPIO thread is only spawned if we were able to acquire the pins
    // If a physical inspection of the device shows no flashing then this state should be obvious.
    if let Ok((door, mut led)) = pins {
        let term = Arc::clone(&term);
        let door_state = door_state.clone();
        let thread = thread::spawn(move || {
            let mut opened_at = None;
            while !term.load(Ordering::Relaxed) {
                let state = door.read().into();

                match (state, opened_at) {
                    (DoorState::Open, None) => opened_at = Some(Instant::now()),
                    (DoorState::Closed, Some(_)) => opened_at = None,
                    _ => {}
                }

                if let Some(opened_at) = opened_at {
                    tx.send(opened_at).expect("send error"); // TODO: Work out how to handle this best
                }

                match state {
                    DoorState::Open => led::flash(&mut led, 1),
                    DoorState::Closed => led::flash(&mut led, 2),
                    DoorState::Unknown => led::flash(&mut led, 3),
                }

                door_state.set_state(state);
                std::thread::sleep(ONE_SECOND);
            }
            eprintln!("GPIO thread exiting");
        });
        threads.push(thread);
    } else {
        eprintln!("unable to set up GPIO")
    }

    // Notification thread
    {
        let term = Arc::clone(&term);
        let thread = thread::spawn(move || {
            // let notified_at = None;
            while !term.load(Ordering::Relaxed) {
                if let Ok(_opened_at) = rx.try_recv() {}
                std::thread::sleep(ONE_SECOND);
            }
            eprintln!("notification thread exiting");
        });
        threads.push(thread);
    }

    // HTTP server shutdown thread
    let server = Arc::new(Server::http("0.0.0.0:8088").unwrap()); // FIXME: unwrap
    {
        let term = Arc::clone(&term);
        let server = Arc::clone(&server);
        let thread = thread::spawn(move || {
            while !term.load(Ordering::Relaxed) {
                std::thread::sleep(ONE_SECOND);
            }
            server.unblock();
            eprintln!("server thread exiting");
        });
        threads.push(thread);
    }

    // HTTP server
    let json = "Content-type: application/json; charset=utf-8"
        .parse::<tiny_http::Header>()
        .unwrap();
    eprintln!("server ready and waiting");
    for request in server.incoming_requests() {
        let response = match request.url() {
            "/" => Response::from_string(door_state.get_state().to_string()),
            "/door.json" => {
                let obj = object! {
                    state: door_state.get_state().to_string(),
                    notified_at: null,
                    open_since: null
                };
                let body = json::stringify_pretty(obj, 2);
                Response::from_string(body).with_header(json.clone())
            }
            _ => Response::from_string("Not found").with_status_code(404),
        };

        request.respond(response).unwrap(); // FIXME: unwrap
    }

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
