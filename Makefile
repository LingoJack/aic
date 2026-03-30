.PHONY: build install uninstall clean

PREFIX ?= $(HOME)/.local/bin

build:
	cargo build --release
	swiftc helpers/indicator.swift -o target/release/aic-indicator -O

install: build
	mkdir -p $(PREFIX)
	cp target/release/aic $(PREFIX)/aic
	cp target/release/aic-indicator $(PREFIX)/aic-indicator

uninstall:
	rm -f $(PREFIX)/aic $(PREFIX)/aic-indicator

clean:
	cargo clean
