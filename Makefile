.PHONY: all bash tcc

all: tcc bash

tcc:
	$(MAKE) -C tinycc

bash:
	$(MAKE) -C bash
