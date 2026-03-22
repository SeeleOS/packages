.PHONY: all basic bash tcc tinycc busybox list

basic: tcc bash busybox

all: tcc bash busybox

tcc:
	cargo run -- install tinycc

tinycc:
	cargo run -- install tinycc

bash:
	cargo run -- install bash

busybox:
	cargo run -- install busybox

list:
	cargo run -- list
