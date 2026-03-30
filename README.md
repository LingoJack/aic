# aic - AI Computer

CLI tool for AI agents to control macOS like a human. Simulate keyboard, mouse, and take screenshots.

## Install

```bash
# Build and install to ~/.local/bin
make install

# Or specify a custom path
make install PREFIX=/usr/local/bin
```

## Permissions

macOS requires explicit permission for input simulation and screen capture:

1. **Accessibility** - System Settings > Privacy & Security > Accessibility
   Required for keyboard and mouse control.

2. **Screen Recording** - System Settings > Privacy & Security > Screen Recording
   Required for screenshots.

Add your terminal app (e.g. Terminal, iTerm2, Warp) to both lists.

## Usage

### Keyboard

```bash
# Press a single key
aic key press enter
aic key press a
aic key press f5

# Key combinations (modifiers first, main key last)
aic key combo cmd c            # Cmd+C (copy)
aic key combo cmd v            # Cmd+V (paste)
aic key combo cmd shift s      # Cmd+Shift+S (save as)
aic key combo alt tab          # Alt+Tab

# Hold and release (for shift-selection, gaming, etc.)
aic key down shift
aic key press right             # Shift+Right: extend selection
aic key up shift
```

### Type Text

```bash
# Type a string character by character
aic type "hello world"

# Supports Unicode
aic type "你好世界"

# Custom typing speed
aic type "slow typing" --delay-ms 100
```

### Mouse

Coordinates are in macOS logical points. (0, 0) is the top-left corner of the main display.

```bash
# Move cursor
aic mouse move 500 300

# Click
aic mouse click 500 300          # left click
aic mouse doubleclick 500 300    # double click
aic mouse rightclick 500 300     # right click

# Long press (hold)
aic mouse longpress 500 300 --duration-ms 1000

# Drag from point A to point B
aic mouse drag 100 200 400 500 --duration-ms 800

# Scroll (positive = up/right, negative = down/left)
aic mouse scroll 0 -5             # scroll down 5 lines
aic mouse scroll 0 3              # scroll up 3 lines
aic mouse scroll -2 0             # scroll left
aic mouse scroll 0 -3 --x 500 --y 300  # scroll at specific position
```

### Screenshot

```bash
# Save to file
aic screenshot -o screen.png

# Output base64-encoded PNG (useful for AI vision APIs)
aic screenshot --base64

# Raw PNG to stdout
aic screenshot > screen.png
```

## Supported Keys

| Category   | Keys                                                      |
|------------|-----------------------------------------------------------|
| Letters    | `a` - `z`                                                 |
| Digits     | `0` - `9`                                                 |
| Modifiers  | `cmd`, `shift`, `alt` / `option`, `ctrl`                  |
| Special    | `enter`, `tab`, `space`, `delete`, `escape`, `capslock`   |
| Arrows     | `up`, `down`, `left`, `right`                             |
| Function   | `f1` - `f12`                                              |
| Navigation | `home`, `end`, `pageup`, `pagedown`                       |
| Symbols    | `comma`, `dot`, `slash`, `backslash`, `semicolon`, `quote`, `backtick` |

## How It Works

`aic` uses macOS Core Graphics (CGEvent) APIs to post synthetic input events at the HID level. This is the same mechanism macOS assistive technologies use, which is why Accessibility permission is required.

Screenshots use `CGDisplayCreateImage` to capture the main display framebuffer.

## License

MIT
