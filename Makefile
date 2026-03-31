.PHONY: build install uninstall clean push

PREFIX ?= $(HOME)/.local/bin

build:
	cargo build --release
	swiftc helpers/indicator.swift -o target/release/aic-indicator -O
	swiftc helpers/ax.swift -o target/release/aic-ax -O -framework Cocoa -framework ApplicationServices

install: build
	mkdir -p $(PREFIX)
	cp target/release/aic $(PREFIX)/aic
	cp target/release/aic-indicator $(PREFIX)/aic-indicator
	cp target/release/aic-ax $(PREFIX)/aic-ax

uninstall:
	rm -f $(PREFIX)/aic $(PREFIX)/aic-indicator $(PREFIX)/aic-ax

clean:
	cargo clean

# Commit and push changes
push:
	@echo "📦 提交更改..."
	@git add -A
	@if [ -z "$$(git status --porcelain)" ]; then \
		echo "ℹ️  没有更改需要提交"; \
	else \
		git commit -m "checkpoint: $$(date '+%Y-%m-%d %H:%M:%S')"; \
		echo "✅ 提交完成"; \
	fi
	@echo "🚀 推送到远程..."
	@git push origin main
