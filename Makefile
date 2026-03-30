.PHONY: build install uninstall clean

PREFIX ?= $(HOME)/.local/bin

build:
	cargo build --release

install: build
	mkdir -p $(PREFIX)
	cp target/release/aic $(PREFIX)/aic

uninstall:
	rm -f $(PREFIX)/aic

clean:
	cargo clean
