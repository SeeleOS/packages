include ../config.mk

PKG_NAME    := busybox
PKG_VERSION := 1.36.1
PKG_SITE    := https://busybox.net/downloads

DISTDIR   := $(BASE)/packages/distfiles
DISTFILE  := $(DISTDIR)/$(PKG_NAME)-$(PKG_VERSION).tar.bz2

WRKDIR    := $(BASE)/packages/work/$(PKG_NAME)
WRKSRC    := $(WRKDIR)/$(PKG_NAME)-$(PKG_VERSION)

PATCHDIR    := $(BASE)/packages/$(PKG_NAME)/patches
CONFIG_FILE := $(BASE)/packages/$(PKG_NAME)/config.seele

FULL_TARGET := $(WRKSRC)/busybox

.PHONY: all fetch extract patch configure relibc build install clean distclean

all: build install

fetch:
	@echo "[packages][$(PKG_NAME)] fetch tarball..."
	@mkdir -p "$(DISTDIR)"
	@if [ ! -f "$(DISTFILE)" ]; then \
	  echo "  downloading $(PKG_SITE)/$(PKG_NAME)-$(PKG_VERSION).tar.bz2"; \
	  curl -L "$(PKG_SITE)/$(PKG_NAME)-$(PKG_VERSION).tar.bz2" -o "$(DISTFILE)"; \
	else \
	  echo "  using existing $(DISTFILE)"; \
	fi

extract: fetch
	@echo "[packages][$(PKG_NAME)] extract to $(WRKDIR)..."
	@rm -rf "$(WRKDIR)"
	@mkdir -p "$(WRKDIR)"
	@tar -C "$(WRKDIR)" -xf "$(DISTFILE)"

patch: extract
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
	@if [ -f "$(CONFIG_FILE)" ]; then \
	  echo "[packages][$(PKG_NAME)] using config.seele"; \
	  cp "$(CONFIG_FILE)" "$(WRKSRC)/.config"; \
	else \
	  echo "[packages][$(PKG_NAME)] no config.seele"; \
	fi

relibc:
	@echo "[packages][$(PKG_NAME)] building relibc..."
	@$(MAKE) -C "$(RELIBC_ROOT)" all

build: configure relibc
	@echo "[packages][$(PKG_NAME)] building BusyBox..."
	@$(MAKE) -C "$(WRKSRC)" \
	  CROSS_COMPILE=x86_64-elf- \
	  -j$$(nproc)

install: build
	@echo "[packages][$(PKG_NAME)] installing $(FULL_TARGET) to $(INSTALL_DIR)/$(PKG_NAME)..."
	@mkdir -p "$(INSTALL_DIR)"
	@sudo rm -f "$(INSTALL_DIR)/$(PKG_NAME)"
	@sudo cp "$(FULL_TARGET)" "$(INSTALL_DIR)/$(PKG_NAME)"
	@echo "[packages][$(PKG_NAME)] generating applet links in $(INSTALL_DIR)..."
	@cd "$(WRKSRC)" && $(MAKE) busybox.links >/dev/null
	@sudo sh -c 'cd "$(INSTALL_DIR)"; \
		for p in `sort "$(WRKSRC)/busybox.links" | uniq`; do \
			app=$$(basename "$$p"); \
			echo "    -> $$app"; \
			ln -sf "$(PKG_NAME)" "$$app"; \
		done'
	@sync
	@ORIG_SIZE=$$(stat -c%s "$(FULL_TARGET)"); \
	 INST_SIZE=$$(sudo stat -c%s "$(INSTALL_DIR)/$(PKG_NAME)"); \
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
