################################################################################
#
# garage-door-monitor
#
################################################################################

GARAGE_DOOR_MONITOR_VERSION = 0.4.0
# GARAGE_DOOR_MONITOR_SOURCE = garage-door-monitor-$(GARAGE_DOOR_MONITOR_VERSION).tar.gz
# https://github.com/wezm/garage-door-monitor/archive/refs/tags/v0.1.0.tar.gz
GARAGE_DOOR_MONITOR_SITE = $(call github,wezm,garage-door-monitor,v$(GARAGE_DOOR_MONITOR_VERSION))
# GARAGE_DOOR_MONITOR_LICENSE = MIT OR Apache-2.0
# GARAGE_DOOR_MONITOR_LICENSE_FILES = COPYING

GARAGE_DOOR_MONITOR_DEPENDENCIES = host-rustc

# CC_ and AR_ For the ring crate
GARAGE_DOOR_MONITOR_CARGO_ENV = CARGO_HOME=$(HOST_DIR)/share/cargo \
    CC_$(subst -,_,$(RUSTC_TARGET_NAME))=$(TARGET_CC) \
    AR_$(subst -,_,$(RUSTC_TARGET_NAME))=$(TARGET_AR)

GARAGE_DOOR_MONITOR_CARGO_OPTS = \
    --target=$(RUSTC_TARGET_NAME) \
    --manifest-path=$(@D)/app/Cargo.toml

ifeq ($(BR2_ENABLE_RUNTIME_DEBUG),y)
GARAGE_DOOR_MONITOR_CARGO_BIN_SUBDIR = debug
else
GARAGE_DOOR_MONITOR_CARGO_BIN_SUBDIR = release
GARAGE_DOOR_MONITOR_CARGO_OPTS += --release
endif

#GARAGE_DOOR_MONITOR_BIN_DIR = target/$(RUSTC_TARGET_NAME)/$(GARAGE_DOOR_MONITOR_CARGO_MODE)
GARAGE_DOOR_MONITOR_BIN_DIR = app/target/$(RUSTC_TARGET_NAME)/$(GARAGE_DOOR_MONITOR_CARGO_BIN_SUBDIR)

define GARAGE_DOOR_MONITOR_BUILD_CMDS
    $(TARGET_MAKE_ENV) $(GARAGE_DOOR_MONITOR_CARGO_ENV) \
            cargo build $(GARAGE_DOOR_MONITOR_CARGO_OPTS)
endef

define GARAGE_DOOR_MONITOR_INSTALL_TARGET_CMDS
    $(INSTALL) -D -m 0755 $(@D)/$(GARAGE_DOOR_MONITOR_BIN_DIR)/garage-door-monitor \
            $(TARGET_DIR)/usr/bin/garage-door-monitor
endef

define GARAGE_DOOR_MONITOR_USERS
        _garage -1 _garage -1 * - - - Garage Door Monitor daemon
endef

$(eval $(generic-package))
