################################################################################
#
# rsdate
#
################################################################################

RSDATE_VERSION = 0.4.0
# RSDATE_SOURCE = rsdate-$(RSDATE_VERSION).tar.gz
# https://github.com/wezm/rsdate/archive/refs/tags/0.1.0.tar.gz
RSDATE_SITE = $(call github,wezm,rsdate,$(RSDATE_VERSION))
RSDATE_LICENSE = MIT OR Apache-2.0
# RSDATE_LICENSE_FILES = COPYING

RSDATE_DEPENDENCIES = host-rustc

# CC_ and AR_ For the ring crate
RSDATE_CARGO_ENV = CARGO_HOME=$(HOST_DIR)/share/cargo \
    CC_$(subst -,_,$(RUSTC_TARGET_NAME))=$(TARGET_CC) \
    AR_$(subst -,_,$(RUSTC_TARGET_NAME))=$(TARGET_AR)

RSDATE_CARGO_OPTS = \
    --target=$(RUSTC_TARGET_NAME) \
    --manifest-path=$(@D)/Cargo.toml

ifeq ($(BR2_ENABLE_RUNTIME_DEBUG),y)
RSDATE_CARGO_BIN_SUBDIR = debug
else
RSDATE_CARGO_BIN_SUBDIR = release
RSDATE_CARGO_OPTS += --release
endif

RSDATE_BIN_DIR = target/$(RUSTC_TARGET_NAME)/$(RSDATE_CARGO_BIN_SUBDIR)

define RSDATE_BUILD_CMDS
    $(TARGET_MAKE_ENV) $(RSDATE_CARGO_ENV) \
            cargo build $(RSDATE_CARGO_OPTS)
endef

define RSDATE_INSTALL_TARGET_CMDS
    $(INSTALL) -D -m 0755 $(@D)/$(RSDATE_BIN_DIR)/rsdate \
            $(TARGET_DIR)/usr/bin/rsdate
endef

$(eval $(generic-package))
