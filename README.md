Garage Door Monitor
===================

[![Build Status](https://api.cirrus-ci.com/github/wezm/garage-door-monitor.svg)](https://cirrus-ci.com/github/wezm/garage-door-monitor)

<!-- TODO: Add photo here -->

A small application that monitors the state of my garage door and sends an
alert to [Mattermost] if it gets left open. The application is deployed to a
minimal custom Linux installation running on a Raspberry Pi Zero W.

The binary that monitors the garage door is in the `app` directory. It is
implemented in [Rust]. The `buildroot` directory contains the [Buildroot]
configuration for building the Raspberry Pi image. The `hardware` directory
contains a wiring diagram and 3D model for a spacer I 3D printed to mount
the reed switch to.

The system image, which includes the [kernel], [busybox], [openntp], [rsdate],
[wpa_supplicant], and the monitoring application itself is about 13Mb. It is
stored on a 256Mb micro SD card, which is the smallest I could get my hands on.
The system boots and is interactive in less than 5 seconds and runs
entirely from RAM (the microSD card is not even mounted).

[kernel]: https://www.kernel.org/
[busybox]: https://www.busybox.net/
[openntp]: https://www.openntpd.org/
[rsdate]: https://github.com/wezm/rsdate
[wpa_supplicant]: https://hostap.epitest.fi/wpa_supplicant/

Building
--------

**Note:** These instructions assume a Linux host.

To build the Linux image you will need to download and extract Buildroot as
well as install [its system requirements][reqs]. I used the [stable 2021.08
release][buildroot-dl].

Clone the repo then populate two configuration files:

* `buildroot/overlay/etc/default/garage-door-monitor` — Web-hook URL. E.g.

      GARAGE_WEBHOOK="https://example.com/"

* `buildroot/overlay/etc/wpa_supplicant.conf` — Wi-Fi configuration. E.g.

      network={
          ssid="NetworkName"
          psk="password"
      }

In the directory of the extracted Buildroot (E.g. `buildroot-2021.08`) load the
configuration:

```
make defconfig BR2_DEFCONFIG=../path/to/garage-door-monitor/buildroot/configs/garage_defconfig
```

Now point Buildroot at the external tree in this repository and kick off the
build. This will build the toolchain, kernel, and root file system. Note that
for subsequent rebuilds you can use `make` without needing to specify
`BR2_EXTERNAL` as this is remembered:

```
make BR2_EXTERNAL=../path/to/garage-door-monitor/buildroot
```

The SD card image is output to: `output/images/sdcard.img`. Write it to an SD
card with `dd` or similar. Be sure to double check the path to the SD card
device (`/dev/sdd` in this case):

```
sudo dd if=output/images/sdcard.img of=/dev/sdd bs=128k
```

Wiring
------

* A reed switch is connected between 3.3V and header pin 38 via a 10kΩ resistor. Internal pull-downs are enabled on the pin.
* The anode of a 3mm LED is connected to header pin 40 via a 220Ω resistor.
* The Pi is powered through header pins 4 (5V) and 6 (GND).

See also the Fritzing wiring diagram in `hardware`.

Licence
-------

This project is dual licenced under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](https://github.com/wezm/garage-door-monitor/blob/master/LICENSE-APACHE))
- MIT license ([LICENSE-MIT](https://github.com/wezm/garage-door-monitor/blob/master/LICENSE-MIT))

at your option.

[Buildroot]: https://buildroot.org/
[buildroot-dl]: https://buildroot.org/downloads/buildroot-2021.08.tar.bz2
[Mattermost]: https://mattermost.com/
[Rust]: https://www.rust-lang.org/
[reqs]: https://buildroot.org/downloads/manual/manual.html#requirement
