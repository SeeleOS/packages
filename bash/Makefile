include ../config.mk

PKG_NAME   := bash
PKG_VER    := 5.3
PKG_TARBALL := bash-$(PKG_VER).tar.gz
PKG_URL     := https://ftp.gnu.org/gnu/bash/$(PKG_TARBALL)

WORKDIR  := $(BASE)/packages/work/$(PKG_NAME)
WORKSRC  := $(WORKDIR)/src
STAMPDIR := $(WORKDIR)/.stamp

PATCHDIR := $(BASE)/packages/$(PKG_NAME)/patches

BUILD_DIR   := $(WORKSRC)/build
FULL_TARGET := $(BUILD_DIR)/bash
FETCH_STAMP := $(STAMPDIR)/fetch
PATCH_STAMP := $(STAMPDIR)/patch
CONFIGURE_STAMP := $(STAMPDIR)/configure

CC      := x86_64-elf-gcc
AR      := x86_64-elf-ar
HOST_CC ?= gcc

RELIBC_INCLUDE := $(RELIBC_ROOT)/target/x86_64-seele/include

CFLAGS_SEELE  := -Wall -O2 \
                 -ffreestanding -D__seele__ -mno-red-zone \
                 -fno-stack-protector -fno-builtin -fno-pie -no-pie \
                 -nostdinc -I$(RELIBC_INCLUDE)

LDFLAGS_SEELE := -static -nostdlib \
                 $(RELIBC_PATH)/crt0.o \
                 $(RELIBC_PATH)/crti.o

LIBS_SEELE    := $(RELIBC_PATH)/libc.a \
                 $(RELIBC_PATH)/crtn.o

BUILD_TRIPLE := $(shell sh $(WORKSRC)/support/config.guess 2>/dev/null || echo x86_64-pc-linux-gnu)
HOST_TRIPLE  ?= x86_64-unknown-none

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
	@cd "$(BUILD_DIR)" && \
	  CC="$(CC)" AR="$(AR)" \
	  CFLAGS="$(CFLAGS_SEELE)" \
	  LDFLAGS="$(LDFLAGS_SEELE)" \
	  LIBS="$(LIBS_SEELE)" \
	  ../configure \
	    --host="$(HOST_TRIPLE)" \
	    --build="$(BUILD_TRIPLE)" \
	    --prefix=/ \
	    --disable-nls \
	    --without-bash-malloc
	@touch "$@"

relibc:
	@echo "[packages][$(PKG_NAME)] building relibc..."
	@$(MAKE) -C "$(RELIBC_ROOT)" all

build: $(CONFIGURE_STAMP) relibc
	@echo "[packages][$(PKG_NAME)] building bash..."
	@cd "$(BUILD_DIR)" && \
	  $(MAKE) \
	    ADDON_LDFLAGS='-Wl,--allow-multiple-definition' \
	    bash
	@echo "[packages][$(PKG_NAME)] build done. Entry point:"
	@readelf -h "$(FULL_TARGET)" | grep Entry || true

install: build
	@echo "[packages][$(PKG_NAME)] installing $(FULL_TARGET) to $(INSTALL_DIR)/bash..."
	@mkdir -p "$(INSTALL_DIR)"
	@sudo rm -f "$(INSTALL_DIR)/bash"
	@sudo cp "$(FULL_TARGET)" "$(INSTALL_DIR)/bash"
	@sync
	@ORIG_SIZE=$$(stat -c%s "$(FULL_TARGET)"); \
	 INST_SIZE=$$(sudo stat -c%s "$(INSTALL_DIR)/bash"); \
	 echo "Local: $$ORIG_SIZE bytes | Installed: $$INST_SIZE bytes"; \
	 if [ "$$ORIG_SIZE" -ne "$$INST_SIZE" ]; then \
	     echo "[packages][$(PKG_NAME)][ERROR]: Size mismatch!"; \
	     exit 1; \
	 else \
	     echo "[packages][$(PKG_NAME)][OK]: Installation verified."; \
	 fi

clean:
	@echo "[packages][$(PKG_NAME)] cleaning $(WORKDIR)..."
	@rm -rf "$(WORKDIR)"

distclean: clean
