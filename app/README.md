Garage Door Monitor Software
============================

This is the brains of the operation. It uses a thread per task
approach where there is a dedicated thread for each of:

- General-purpose input/output (GPIO)
  - Monitoring the door state and controlling the status LED.
- State management
  - Receives updates from the GPIO thread via a channel and detects door state
    transitions.
  - Updates the global state when a change is detected and notes the time the
    door was opened if the state transitions to open.
- Notification thread
  - Checks the global state every 5 seconds and sends a notification if the
    door has been open for more than 5 minutes.
  - Notes the notification time to prevent repeatedly sending the notification.
- HTTP server
  - Serves the web page and JSON end point.
- Main thread
  - Polls a flag every 5 seconds in order to detect signals and shutdown
    cleanly.
  - Mainly useful during development.

## Building

Buildroot normally takes care of building this application for inclusion in the
Linux image but for development. It can be built manually with `cargo build`.

It's designed in a way that it will still run if it's unable to acquire the
GPIO pins. This allowed me to build and run the rest of the functionality
directly on my Linux PC before cross-compiling for the Pi later.
