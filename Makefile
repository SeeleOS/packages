.PHONY: all bash tcc busybox

basic: tcc bash busybox

all: tcc bash busybox

tcc:
	$(MAKE) -C tinycc

bash:
	$(MAKE) -C bash

busybox:
	$(MAKE) -C busybox
