include ../config.mk

PKG_NAME    := busybox
PKG_VER     := 1.37.0
PKG_TARBALL := busybox-$(PKG_VER).tar.bz2
PKG_URL     := https://busybox.net/downloads/$(PKG_TARBALL)

WORKDIR   := $(BASE)/packages/work/$(PKG_NAME)
WORKSRC   := $(WORKDIR)/src
STAMPDIR  := $(WORKDIR)/.stamp
PATCHDIR  := $(BASE)/packages/$(PKG_NAME)/patches
CONFIG_IN := $(BASE)/packages/$(PKG_NAME)/seele.config

BUILD_DIR       := $(WORKSRC)/build
FULL_TARGET     := $(BUILD_DIR)/busybox
BUILD_CONFIG    := $(BUILD_DIR)/seele.config
FETCH_STAMP     := $(STAMPDIR)/fetch
PATCH_STAMP     := $(STAMPDIR)/patch
CONFIGURE_STAMP := $(STAMPDIR)/configure

CC      := x86_64-elf-gcc
AR      := x86_64-elf-ar
HOST_CC ?= gcc

RELIBC_INCLUDE := $(RELIBC_ROOT)/target/x86_64-seele/include
STARTUP_OBJS_SEELE := -l:crt0.o -l:crti.o

CFLAGS_SEELE := -Wall -O2 \
	-ffreestanding -mno-sse -mno-red-zone \
	-fno-stack-protector -fno-builtin -fno-pie -no-pie \
	-nostdinc -I$(RELIBC_INCLUDE) \
	-D__seele__ -D_GNU_SOURCE

LDFLAGS_SEELE := -static -nostdlib \
	-L$(RELIBC_PATH)

LDLIBS_SEELE := -l:libc.a -l:crtn.o

.PHONY: all fetch patch configure relibc build install clean distclean

all: build install

fetch: $(FETCH_STAMP)

$(FETCH_STAMP):
	@echo "[packages][$(PKG_NAME)] fetching sources..."
	@mkdir -p "$(WORKDIR)" "$(STAMPDIR)"
	@if [ ! -d "$(WORKSRC)" ]; then \
	  echo "  downloading $(PKG_URL)..."; \
	  rm -rf "$(WORKSRC)" "$(WORKDIR)/$(PKG_TARBALL)"; \
	  cd "$(WORKDIR)" && \
	    if command -v curl >/dev/null 2>&1; then \
	      curl -L -o "$(PKG_TARBALL)" "$(PKG_URL)"; \
	    elif command -v wget >/dev/null 2>&1; then \
	      wget -O "$(PKG_TARBALL)" "$(PKG_URL)"; \
	    else \
	      echo "[ERROR] neither curl nor wget found; cannot download tarball"; \
	      exit 1; \
	    fi && \
	    mkdir -p src && \
	    tar -xf "$(PKG_TARBALL)" --strip-components=1 -C src; \
	else \
	  echo "  reusing existing source tree at $(WORKSRC)"; \
	fi
	@touch "$@"

patch: $(PATCH_STAMP)

$(PATCH_STAMP): $(FETCH_STAMP) $(wildcard $(PATCHDIR)/*.patch)
	@echo "[packages][$(PKG_NAME)] applying patches from $(PATCHDIR)..."
	@mkdir -p "$(STAMPDIR)"
	@rm -f "$(CONFIGURE_STAMP)"
	@if ls "$(PATCHDIR)"/*.patch >/dev/null 2>&1; then \
	  cd "$(WORKSRC)"; \
	  for p in "$(PATCHDIR)"/*.patch; do \
	    echo "  -> $$p"; \
	    patch -N -p1 < "$$p" || { echo "[ERROR] patch $$p failed"; exit 1; }; \
	  done; \
	else \
	  echo "  no patches"; \
	fi
	@touch "$@"

configure: $(CONFIGURE_STAMP)

$(CONFIGURE_STAMP): $(PATCH_STAMP)
	@echo "[packages][$(PKG_NAME)] configuring..."
	@mkdir -p "$(BUILD_DIR)" "$(STAMPDIR)"
	@cp "$(CONFIG_IN)" "$(BUILD_CONFIG)"
	@printf '%s\n' \
	  'CONFIG_EXTRA_CFLAGS="$(CFLAGS_SEELE)"' \
	  'CONFIG_EXTRA_LDFLAGS="$(LDFLAGS_SEELE)"' \
	  'CONFIG_EXTRA_LDLIBS="$(LDLIBS_SEELE)"' \
	  >> "$(BUILD_CONFIG)"
	@KCONFIG_ALLCONFIG="$(BUILD_CONFIG)" \
		$(MAKE) -C "$(WORKSRC)" O="$(BUILD_DIR)" HOSTCC="$(HOST_CC)" allnoconfig
	@touch "$@"

relibc:
	@echo "[packages][$(PKG_NAME)] checking relibc artifacts..."
	@test -d "$(RELIBC_INCLUDE)"
	@test -f "$(RELIBC_PATH)/crt0.o"
	@test -f "$(RELIBC_PATH)/crti.o"
	@test -f "$(RELIBC_PATH)/crtn.o"
	@test -f "$(RELIBC_PATH)/libc.a"

build: $(CONFIGURE_STAMP) relibc
	@echo "[packages][$(PKG_NAME)] building busybox..."
	@$(MAKE) -C "$(WORKSRC)" O="$(BUILD_DIR)" \
		HOSTCC="$(HOST_CC)" \
		CC="$(CC)" \
		AR="$(AR)" \
		CROSS_COMPILE="$(CC:%gcc=%)" \
		CFLAGS_busybox="$(STARTUP_OBJS_SEELE)" \
		busybox
	@echo "[packages][$(PKG_NAME)] build done. Entry point:"
	@readelf -h "$(FULL_TARGET)" | grep Entry || true

install: build
	@echo "[packages][$(PKG_NAME)] installing busybox symlinks into $(INSTALL_DIR)..."
	@mkdir -p "$(INSTALL_DIR)"
	@cd "$(BUILD_DIR)" && \
		sudo "$(WORKSRC)/applets/install.sh" "$(INSTALL_DIR)" --cleanup || true
	@cd "$(BUILD_DIR)" && \
		sudo "$(WORKSRC)/applets/install.sh" "$(INSTALL_DIR)" --symlinks
	@sync
	@test -f "$(INSTALL_DIR)/bin/busybox"
	@test -L "$(INSTALL_DIR)/bin/ls"
	@echo "[packages][$(PKG_NAME)][OK]: busybox and symlink applets installed."

clean:
	@echo "[packages][$(PKG_NAME)] cleaning $(WORKDIR)..."
	@rm -rf "$(WORKDIR)"

distclean: clean
