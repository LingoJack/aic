# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## What is this?

`aic` (AI Computer) is a macOS CLI tool that lets AI agents control the computer like a human — simulate keyboard/mouse input and capture screenshots. Built in Rust with a Swift helper for visual indicators.

## Build & Install

```bash
make build     # cargo build --release + swiftc indicator helper
make install   # copies aic + aic-indicator to ~/.local/bin
make clean     # cargo clean
```

Two binaries are produced:
- `target/release/aic` — main Rust CLI
- `target/release/aic-indicator` — Swift visual overlay (orange circle on click)

## Architecture

```
main.rs → cli.rs (clap derive parsing)
  ├→ keyboard.rs  — CGEvent keyboard simulation (uses keymap.rs for name→keycode)
  ├→ mouse.rs     — CGEvent mouse simulation + custom FFI for scroll wheel
  ├→ screenshot.rs — CGDisplay screen capture, BGRA→RGBA, PNG/base64 output
  ├→ preview.rs    — dry-run: captures screenshot + draws annotation overlays
  ├→ indicator.rs  — spawns aic-indicator subprocess (fire-and-forget)
  └→ error.rs      — AicError enum
helpers/
  └→ indicator.swift — borderless NSWindow, orange circle, fade-out animation
```

### Key technical details

- **Keyboard events**: `CGEvent::new_keyboard_event` posted at `CGEventTapLocation::HID`. Modifiers set via `CGEventFlags`. Unicode typing uses `set_string_from_utf16_unchecked` on a dummy keycode-0 event (bypasses keymap).
- **Scroll wheel**: `core-graphics 0.24` doesn't expose scroll events — `mouse.rs` has custom `extern "C"` FFI to `CGEventCreateScrollWheelEvent`.
- **Retina handling**: `preview.rs` calculates scale factor (`pixel_width / point_width`) and multiplies all drawing coordinates. Mouse coordinates are always in logical points.
- **Indicator**: non-blocking subprocess spawn. Searches for `aic-indicator` next to the executable, then falls back to PATH.
- **Preview mode**: annotates a screenshot without executing the action. Default output is base64 PNG to stdout (designed for multimodal LLM consumption).

## macOS permissions required

- **Accessibility** (System Settings > Privacy & Security) — for keyboard/mouse simulation
- **Screen Recording** — for screenshots and preview
