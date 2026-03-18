.PHONY: all example

all: example busybox

example:
	$(MAKE) -C example

busybox:
	$(MAKE) -C busybox

tcc:
	$(MAKE) -C tinycc
