#!/bin/sh
PJT_ROOT="$HOME/Projects/garage-door-monitor/buildroot"
git -C "$PJT_ROOT" rev-parse --short HEAD > "$PJT_ROOT/overlay/root/current.version"

