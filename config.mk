BASE         := $(abspath ../..)

RELIBC_ROOT  := $(BASE)/relibc-seele
RELIBC_PATH  := $(RELIBC_ROOT)/target/x86_64-seele/release

SYSROOT      := $(BASE)/sysroot
INSTALL_DIR  := $(SYSROOT)/programs

TRIPLE := x86_64-seele

CC 	     := clang --target=$(TRIPLE)
