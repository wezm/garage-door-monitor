#!/bin/sh
mkdir -p "$BR2_EXTERNAL_WEZM_PATH/overlay/root"
git -C "$BR2_EXTERNAL_WEZM_PATH" rev-parse --short HEAD > "$BR2_EXTERNAL_WEZM_PATH/overlay/root/current.version"
