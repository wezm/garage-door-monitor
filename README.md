# Garage Door Monitor

[![Build Status](https://api.cirrus-ci.com/github/wezm/garage-door-monitor.svg)](https://cirrus-ci.com/github/wezm/garage-door-monitor)

<!-- TODO: Add photo here -->

A small application that monitors the state of my garage door and sends an
alert to [Mattermost] if it gets left open. The application is deployed to a
minimal custom Linux installation running on a Raspberry Pi Zero W.

The binary that monitors the garage door is in the `app` directory. It is
implemented in [Rust]. The `buildroot` directory contains the [Buildroot]
configuration for building the Raspberry Pi image. The image is about 30Mb
and runs entirely from RAM.

[Buildroot]: https://buildroot.org/
[Mattermost]: https://mattermost.com/
[Rust]: https://www.rust-lang.org/
