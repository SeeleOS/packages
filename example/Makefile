include ../config.mk

PKG_NAME    := example
PKG_GIT     := https://github.com/SeeleOS/example

# 工作目录：源码放在 packages/work/example/src
WRKDIR      := $(BASE)/packages/work/$(PKG_NAME)
SRC_DIR     := $(WRKDIR)/src
BUILD_DIR   := $(SRC_DIR)
FULL_TARGET := $(BUILD_DIR)/example

.PHONY: all fetch build install clean distclean

all: build install

fetch:
	@echo "[packages][$(PKG_NAME)] fetch from $(PKG_GIT)..."
	@mkdir -p "$(WRKDIR)"
	@if [ -d "$(SRC_DIR)/.git" ]; then \
	  echo "  updating existing clone..."; \
	  cd "$(SRC_DIR)" && git pull --ff-only; \
	else \
	  echo "  cloning fresh repo..."; \
	  rm -rf "$(SRC_DIR)"; \
	  git clone "$(PKG_GIT)" "$(SRC_DIR)"; \
	fi

build: fetch
	@echo "[packages][$(PKG_NAME)] building..."
	@$(MAKE) -C "$(BUILD_DIR)"

install: build
	@echo "[packages][$(PKG_NAME)] installing to $(INSTALL_DIR)/$(PKG_NAME)..."
	@mkdir -p "$(INSTALL_DIR)"
	@sudo rm -f "$(INSTALL_DIR)/$(PKG_NAME)"
	@sudo cp "$(FULL_TARGET)" "$(INSTALL_DIR)/$(PKG_NAME)"
	@sync
	@ORIG_SIZE=$$(stat -c%s "$(FULL_TARGET)"); \
	 INST_SIZE=$$(sudo stat -c%s "$(INSTALL_DIR)/$(PKG_NAME)" ); \
	 echo "Local: $$ORIG_SIZE bytes | Installed: $$INST_SIZE bytes"; \
	 if [ "$$ORIG_SIZE" -ne "$$INST_SIZE" ]; then \
	     echo "[ERROR]: Size mismatch!"; \
	     exit 1; \
	 else \
	     echo "[OK]: Installation verified."; \
	 fi

clean:
	@echo "[packages][$(PKG_NAME)] cleaning build artifacts..."
	@if [ -d "$(BUILD_DIR)" ]; then \
	  $(MAKE) -C "$(BUILD_DIR)" clean || true; \
	fi

distclean:
	@echo "[packages][$(PKG_NAME)] removing work dir $(WRKDIR)..."
	@rm -rf "$(WRKDIR)"
