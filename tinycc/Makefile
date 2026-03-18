include ../config.mk

PKG_NAME   := tinycc
PKG_GIT    := https://github.com/TinyCC/tinycc.git
PKG_COMMIT := fada98b

WRKDIR  := $(BASE)/packages/work/$(PKG_NAME)
WRKSRC  := $(WRKDIR)/src

PATCHDIR := $(BASE)/packages/$(PKG_NAME)/patches

BUILD_DIR   := $(WRKSRC)/build
FULL_TARGET := $(BUILD_DIR)/tcc

CC      := x86_64-elf-gcc
AR      := x86_64-elf-ar
HOST_CC ?= gcc

RELIBC_INCLUDE := $(RELIBC_ROOT)/target/x86_64-seele/include

CFLAGS_SEELE  := -Wall -O2 \
                 -ffreestanding -D__seele__ -mno-red-zone \
                 -fno-stack-protector -fno-builtin -fno-pie -no-pie \
                 -nostdinc -I$(RELIBC_INCLUDE) \
                 -I$(WRKSRC) -I$(WRKSRC)/include

LDFLAGS_SEELE := -static -nostdlib \
                 $(RELIBC_PATH)/crt0.o \
                 $(RELIBC_PATH)/crti.o \
                 $(RELIBC_PATH)/libc.a \
                 $(RELIBC_PATH)/crtn.o

.PHONY: all fetch patch configure relibc build install clean distclean

all: build install

fetch:
	@echo "[packages][$(PKG_NAME)] fetch from git..."
	@mkdir -p "$(WRKDIR)"
	@if [ -d "$(WRKSRC)/.git" ]; then \
	  echo "  updating existing clone..."; \
	  cd "$(WRKSRC)" && \
	    git fetch origin --tags && \
	    git reset --hard "$(PKG_COMMIT)"; \
	else \
	  echo "  cloning fresh repo..."; \
	  rm -rf "$(WRKSRC)"; \
	  git clone "$(PKG_GIT)" "$(WRKSRC)"; \
	  cd "$(WRKSRC)" && git checkout "$(PKG_COMMIT)"; \
	fi

patch: fetch
	@echo "[packages][$(PKG_NAME)] applying patches from $(PATCHDIR)..."
	@if ls "$(PATCHDIR)"/*.patch >/dev/null 2>&1; then \
	  cd "$(WRKSRC)"; \
	  for p in "$(PATCHDIR)"/*.patch; do \
	    echo "  -> $$p"; \
	    patch -p1 < "$$p" || { echo "[ERROR] patch $$p failed"; exit 1; }; \
	  done; \
	else \
	  echo "  no patches"; \
	fi

configure: patch
	@echo "[packages][$(PKG_NAME)] configuring..."
	@cd "$(WRKSRC)" && \
	  if [ ! -f config.mak ]; then \
	    ./configure --prefix=/; \
	  fi

relibc:
	@echo "[packages][$(PKG_NAME)] building relibc..."
	@$(MAKE) -C "$(RELICB_ROOT)" all

build: configure relibc
	@echo "[packages][$(PKG_NAME)] building TinyCC..."
	@cd "$(WRKSRC)" && \
	  if [ ! -x ./c2str.exe ]; then \
	    echo "Building host tool c2str.exe..."; \
	    $(HOST_CC) -DC2STR conftest.c -o c2str.exe; \
	  fi && \
	  if [ ! -f tccdefs_.h ]; then \
	    echo "Generating tccdefs_.h..."; \
	    ./c2str.exe include/tccdefs.h tccdefs_.h; \
	  fi && \
	  mkdir -p "$(BUILD_DIR)" && \
	  $(MAKE) -f Makefile \
	    CC="$(CC)" AR="$(AR)" \
	    LIBS="" \
	    CFLAGS="$(CFLAGS_SEELE)" \
	    LDFLAGS="$(LDFLAGS_SEELE)" \
	    tcc && \
	  mv tcc "$(FULL_TARGET)"
	@echo "Build Done. Entry Point:"
	@readelf -h "$(FULL_TARGET)" | grep Entry || true

install: build
	@echo "[packages][$(PKG_NAME)] installing $(FULL_TARGET) to $(INSTALL_DIR)/tcc..."
	@mkdir -p "$(INSTALL_DIR)"
	@sudo rm -f "$(INSTALL_DIR)/tcc"
	@sudo cp "$(FULL_TARGET)" "$(INSTALL_DIR)/tcc"
	@sync
	@ORIG_SIZE=$$(stat -c%s "$(FULL_TARGET)"); \
	 INST_SIZE=$$(sudo stat -c%s "$(INSTALL_DIR)/tcc"); \
	 echo "Local: $$ORIG_SIZE bytes | Installed: $$INST_SIZE bytes"; \
	 if [ "$$ORIG_SIZE" -ne "$$INST_SIZE" ]; then \
	     echo "[packages][$(PKG_NAME)][ERROR]: Size mismatch!"; \
	     exit 1; \
	 else \
	     echo "[packages][$(PKG_NAME)][OK]: Installation verified."; \
	 fi

clean:
	@echo "[packages][$(PKG_NAME)] cleaning $(WRKDIR)..."
	@rm -rf "$(WRKDIR)"

distclean: clean
