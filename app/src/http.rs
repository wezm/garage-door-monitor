use std::borrow::Cow;
use std::error::Error;
use std::net::ToSocketAddrs;
use std::sync::{Arc, RwLock};
use std::time::Instant;

use json::object;
use systemstat::{ByteSize, Platform, System};
use tiny_http::Response;

use crate::uptime::Uptime;
use crate::{DoorState, State};

const HTML: &str = include_str!("home.html");

macro_rules! check_read {
    ($expr:expr) => {
        match $expr {
            std::result::Result::Ok(state) => state,
            std::result::Result::Err(err) => {
                eprintln!("error reading state: {}", err);
                return;
            }
        }
    };
}

pub struct Server(tiny_http::Server);

impl Server {
    pub fn new<A>(addr: A) -> Result<Server, Box<dyn Error + Send + Sync + 'static>>
    where
        A: ToSocketAddrs,
    {
        tiny_http::Server::http(addr).map(Server)
    }

    pub fn handle_requests(&self, state: Arc<RwLock<State>>) {
        // NOTE(unwrap): These are known valid
        let json = "Content-type: application/json; charset=utf-8"
            .parse::<tiny_http::Header>()
            .unwrap();
        let html_content = "Content-type: text/html; charset=utf-8"
            .parse::<tiny_http::Header>()
            .unwrap();
        for request in self.0.incoming_requests() {
            let response = match request.url() {
                "/" => {
                    let current_state = check_read!(state.read());
                    let status = match current_state.door_state {
                        DoorState::Open => {
                            let duration = current_state
                                .open_since
                                .map(|opened| {
                                    let now = Instant::now();
                                    let duration = now.duration_since(opened);
                                    let formatter = timeago::Formatter::new();
                                    Cow::from(formatter.convert(duration))
                                })
                                .unwrap_or_else(|| Cow::from("at an unknown time"));
                            String::from(format!("🔴 Opened {}", duration))
                        }
                        DoorState::Closed => String::from("🟢 Closed"),
                        DoorState::Unknown => String::from("🔵 Unknown"),
                    };
                    let html = HTML.replace("$doorstate$", &status);

                    // Fetch system info
                    let system = System::new();
                    let uptime = system
                        .uptime()
                        .ok()
                        .map(|uptime| Uptime::new(uptime.as_secs()))
                        .unwrap_or_default();
                    let html = html.replace("$uptime$", &uptime.to_string());

                    let memory = system
                        .memory()
                        .map(|mem| {
                            let used =
                                ByteSize(mem.total.as_u64().saturating_sub(mem.free.as_u64()));
                            format!("{} used {} free {} total", used, mem.free, mem.total)
                        })
                        .unwrap_or_else(|_| String::from("unknown"));
                    let html = html.replace("$memory$", &memory);

                    Response::from_string(html).with_header(html_content.clone())
                }
                "/door.json" => {
                    let now = Instant::now();
                    let current_state = check_read!(state.read());
                    let obj = object! {
                        state: current_state.door_state.to_string(),
                        secs_since_notified: current_state.notified_at.map(|notified| now.duration_since(notified).as_secs()),
                        open_for: current_state.open_since.map(|opened| now.duration_since(opened).as_secs())
                    };
                    let body = json::stringify_pretty(obj, 2);
                    Response::from_string(body).with_header(json.clone())
                }
                _ => Response::from_string("Not found").with_status_code(404),
            };

            // Ignoring I/O errors that occur here so that we don't take down the process if there
            // is an issue sending the response.
            let _ = request.respond(response);
        }
    }

    pub fn shutdown(&self) {
        self.0.unblock();
    }
}
