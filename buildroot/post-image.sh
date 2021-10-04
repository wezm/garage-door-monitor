#!/bin/sh
PJT_ROOT="$HOME/Projects/garage-door-monitor/buildroot"
SIZE_RFS=$(stat -c %s ./output/images/rootfs.cpio)
sed -e "s/_size_/$SIZE_RFS/g" "$PJT_ROOT/cmdline.txt.template" > "$PJT_ROOT/cmdline.txt"
cp ./output/images/zImage "$PJT_ROOT/sd_boot/"
cp ./output/images/rootfs.cpio.gz "$PJT_ROOT/sd_boot/"
cp ./output/images/bcm2708-rpi-zero-w.dtb "$PJT_ROOT/sd_boot/"
cp -r ./output/images/rpi-firmware/* "$PJT_ROOT/sd_boot/"
cp "$PJT_ROOT"/*.txt "$PJT_ROOT/sd_boot/" # config.txt and cmdline.txt
