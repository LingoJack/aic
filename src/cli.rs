use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "aic",
    version,
    about = "AI Computer - CLI tool for AI agents to control macOS like a human",
    long_about = "AI Computer (aic) lets AI agents simulate keyboard and mouse input on macOS.\n\n\
        It can press keys, type text, move/click the mouse, and take screenshots.\n\
        Requires Accessibility permission in System Settings > Privacy & Security.\n\n\
        Examples:\n  \
          aic key press enter\n  \
          aic key combo cmd c\n  \
          aic type \"hello world\"\n  \
          aic mouse click 100 200\n  \
          aic screenshot -o screen.png",
    after_help = "Supported key names:\n  \
        Letters:    a-z\n  \
        Digits:     0-9\n  \
        Modifiers:  cmd, shift, alt/option, ctrl\n  \
        Special:    enter, tab, space, delete, escape, capslock\n  \
        Arrow:      up, down, left, right\n  \
        Function:   f1-f12\n  \
        Navigation: home, end, pageup, pagedown\n  \
        Symbols:    comma, dot, slash, backslash, semicolon, quote, backtick\n\n\
        Mouse coordinates use macOS logical points (not Retina pixels).\n\
        Screenshot requires Screen Recording permission."
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Simulate keyboard input (press, combo, hold)
    #[command(
        long_about = "Simulate keyboard input.\n\n\
            Examples:\n  \
              aic key press enter        # press Enter\n  \
              aic key press a            # press 'a'\n  \
              aic key combo cmd c        # Cmd+C (copy)\n  \
              aic key combo cmd shift s  # Cmd+Shift+S\n  \
              aic key down shift         # hold Shift\n  \
              aic key up shift           # release Shift"
    )]
    Key {
        #[command(subcommand)]
        action: KeyAction,
    },

    /// Type a string of text character by character
    #[command(
        long_about = "Type a string of text by simulating keystrokes for each character.\n\
            Supports Unicode (Chinese, emoji, etc.).\n\n\
            Examples:\n  \
              aic type \"hello world\"\n  \
              aic type \"你好\" --delay-ms 50"
    )]
    Type {
        /// The text string to type
        text: String,
        /// Delay between each keystroke in milliseconds
        #[arg(long, default_value = "12")]
        delay_ms: u64,
    },

    /// Simulate mouse actions (click, drag, scroll, etc.)
    #[command(
        long_about = "Simulate mouse input.\n\n\
            Coordinates are absolute screen positions in logical points (top-left is 0,0).\n\n\
            Examples:\n  \
              aic mouse move 100 200                          # move cursor\n  \
              aic mouse click 100 200                         # left click\n  \
              aic mouse doubleclick 100 200                   # double click\n  \
              aic mouse rightclick 100 200                    # right click\n  \
              aic mouse longpress 100 200 --duration-ms 1000  # hold 1s\n  \
              aic mouse drag 100 200 300 400                  # drag\n  \
              aic mouse scroll 0 -3                           # scroll down 3 lines\n  \
              aic mouse scroll 0 3 --x 500 --y 300            # scroll at position\n  \
              aic mouse preview click 500 300 -o p.png        # dry-run preview"
    )]
    Mouse {
        #[command(subcommand)]
        action: MouseAction,
    },

    /// Capture the screen to a file or stdout
    #[command(
        long_about = "Take a screenshot of the main display.\n\
            Requires Screen Recording permission in System Settings.\n\n\
            Examples:\n  \
              aic screenshot -o screen.png    # save to file\n  \
              aic screenshot --base64         # output base64 PNG to stdout\n  \
              aic screenshot > screen.png     # raw PNG bytes to stdout\n  \
              aic screenshot --som -o som.png # SoM annotated screenshot\n  \
              aic screenshot --som            # SoM with base64 output + JSON index to stderr"
    )]
    Screenshot {
        /// Save screenshot to this file path (e.g. screen.png)
        #[arg(short, long)]
        output: Option<String>,
        /// Print base64-encoded PNG to stdout (useful for piping to AI)
        #[arg(long)]
        base64: bool,
        /// Overlay Set-of-Mark numbered labels on interactive UI elements
        #[arg(long)]
        som: bool,
        /// Target application name for SoM (default: frontmost app)
        #[arg(long)]
        app: Option<String>,
    },

    /// Query the Accessibility element tree of an application
    #[command(
        long_about = "Query the macOS Accessibility API to get the UI element tree.\n\
            Requires Accessibility permission in System Settings.\n\n\
            Examples:\n  \
              aic ax                        # full tree of frontmost app\n  \
              aic ax --app Finder --depth 3 # Finder, 3 levels deep\n  \
              aic ax --clickable            # only interactive elements"
    )]
    Ax {
        /// Target application name (default: frontmost app)
        #[arg(long)]
        app: Option<String>,
        /// Maximum tree depth to traverse
        #[arg(long)]
        depth: Option<u32>,
        /// Only show interactive/clickable elements
        #[arg(long)]
        clickable: bool,
    },

    /// Search for UI elements by text content
    #[command(
        long_about = "Search the Accessibility tree for elements matching a text query.\n\
            Searches title, description, and value attributes (case-insensitive).\n\n\
            Examples:\n  \
              aic find \"OK\"                 # find buttons/elements with 'OK'\n  \
              aic find \"Save\" --app Safari  # search in Safari\n  \
              aic find \"Submit\" --role AXButton # only buttons"
    )]
    Find {
        /// Text to search for in element titles, descriptions, and values
        query: String,
        /// Target application name (default: frontmost app)
        #[arg(long)]
        app: Option<String>,
        /// Filter by accessibility role (e.g. AXButton, AXTextField)
        #[arg(long)]
        role: Option<String>,
    },
}

#[derive(Subcommand)]
pub enum KeyAction {
    /// Press and release a single key
    #[command(long_about = "Press and release a single key.\n\n\
        Examples:\n  aic key press enter\n  aic key press a\n  aic key press f5\n  aic key press space")]
    Press {
        /// Key name: a-z, 0-9, enter, tab, space, escape, f1-f12, up/down/left/right, etc.
        key: String,
    },

    /// Press a key combination with modifiers
    #[command(long_about = "Press a key combination. List modifiers first, main key last.\n\n\
        Modifiers: cmd, shift, alt (option), ctrl\n\n\
        Examples:\n  aic key combo cmd c          # Cmd+C\n  \
              aic key combo cmd shift s    # Cmd+Shift+S\n  \
              aic key combo alt tab        # Alt+Tab\n  \
              aic key combo ctrl alt delete # Ctrl+Alt+Delete")]
    Combo {
        /// Keys: modifiers first, main key last (e.g. cmd shift s)
        keys: Vec<String>,
    },

    /// Hold a key down (pair with 'up' to release)
    #[command(long_about = "Hold a key down without releasing.\n\
        Use 'aic key up <key>' to release it later.\n\n\
        Example:\n  aic key down shift\n  aic key press a      # types 'A'\n  aic key up shift")]
    Down {
        /// Key name to hold down
        key: String,
    },

    /// Release a previously held key
    #[command(long_about = "Release a key that was held down with 'aic key down'.\n\n\
        Example:\n  aic key up shift")]
    Up {
        /// Key name to release
        key: String,
    },
}

#[derive(Subcommand)]
pub enum MouseAction {
    /// Move cursor to absolute screen position
    #[command(long_about = "Move the mouse cursor to an absolute screen position.\n\n\
        Example:\n  aic mouse move 500 300")]
    Move {
        /// X coordinate (pixels from left)
        x: f64,
        /// Y coordinate (pixels from top)
        y: f64,
    },

    /// Left-click at position
    #[command(long_about = "Perform a left mouse click at the given position.\n\n\
        Example:\n  aic mouse click 500 300")]
    Click {
        /// X coordinate
        x: f64,
        /// Y coordinate
        y: f64,
    },

    /// Double-click at position
    #[command(long_about = "Perform a double-click at the given position.\n\
        Useful for selecting words in text or opening files.\n\n\
        Example:\n  aic mouse doubleclick 500 300")]
    Doubleclick {
        /// X coordinate
        x: f64,
        /// Y coordinate
        y: f64,
    },

    /// Right-click at position
    #[command(long_about = "Perform a right-click (context menu) at the given position.\n\n\
        Example:\n  aic mouse rightclick 500 300")]
    Rightclick {
        /// X coordinate
        x: f64,
        /// Y coordinate
        y: f64,
    },

    /// Long press (hold) at position
    #[command(long_about = "Press and hold the left mouse button at the given position.\n\
        Useful for triggering long-press menus or hold-to-reveal UI.\n\n\
        Examples:\n  aic mouse longpress 500 300                    # hold 500ms (default)\n  \
              aic mouse longpress 500 300 --duration-ms 2000 # hold 2 seconds")]
    Longpress {
        /// X coordinate
        x: f64,
        /// Y coordinate
        y: f64,
        /// How long to hold in milliseconds
        #[arg(long, default_value = "500")]
        duration_ms: u64,
    },

    /// Drag from (x1,y1) to (x2,y2)
    #[command(long_about = "Click and drag from one position to another.\n\
        Useful for moving windows, selecting text regions, or drawing.\n\n\
        Examples:\n  aic mouse drag 100 200 400 500                  # drag with default speed\n  \
              aic mouse drag 100 200 400 500 --duration-ms 1000 # slow drag over 1s")]
    Drag {
        /// Start X
        x1: f64,
        /// Start Y
        y1: f64,
        /// End X
        x2: f64,
        /// End Y
        y2: f64,
        /// Drag duration in milliseconds
        #[arg(long, default_value = "500")]
        duration_ms: u64,
    },

    /// Scroll the mouse wheel
    #[command(allow_negative_numbers = true, long_about = "Scroll the mouse wheel at the current or specified position.\n\
        Positive dy = scroll up, negative dy = scroll down.\n\
        Positive dx = scroll right, negative dx = scroll left.\n\n\
        Examples:\n  aic mouse scroll 0 -5                        # scroll down 5 lines\n  \
              aic mouse scroll 0 3                         # scroll up 3 lines\n  \
              aic mouse scroll -2 0                        # scroll left\n  \
              aic mouse scroll 0 -3 --x 500 --y 300       # scroll down at position")]
    Scroll {
        /// Horizontal scroll (positive = right, negative = left)
        dx: i32,
        /// Vertical scroll (positive = up, negative = down)
        dy: i32,
        /// Optional: X position to scroll at
        #[arg(long)]
        x: Option<f64>,
        /// Optional: Y position to scroll at
        #[arg(long)]
        y: Option<f64>,
    },

    /// Dry-run preview: annotate a screenshot without executing
    #[command(
        long_about = "Preview a mouse action by drawing it on a screenshot, without actually\n\
            performing it. Useful for LLMs to verify coordinates before acting.\n\n\
            Outputs an annotated screenshot (base64 PNG to stdout by default).\n\n\
            Examples:\n  aic mouse preview click 500 300               # base64 to stdout\n  \
              aic mouse preview click 500 300 -o p.png     # save to file\n  \
              aic mouse preview drag 100 200 400 500       # shows path arrow\n  \
              aic mouse preview scroll 0 -3 --x 500 --y 300"
    )]
    Preview {
        #[command(subcommand)]
        action: PreviewAction,
    },
}

#[derive(Subcommand)]
pub enum PreviewAction {
    /// Preview a click at position
    Click {
        x: f64,
        y: f64,
        /// Save annotated screenshot to file (default: base64 to stdout)
        #[arg(short, long)]
        output: Option<String>,
    },
    /// Preview a double-click at position
    Doubleclick {
        x: f64,
        y: f64,
        #[arg(short, long)]
        output: Option<String>,
    },
    /// Preview a right-click at position
    Rightclick {
        x: f64,
        y: f64,
        #[arg(short, long)]
        output: Option<String>,
    },
    /// Preview a cursor move to position
    Move {
        x: f64,
        y: f64,
        #[arg(short, long)]
        output: Option<String>,
    },
    /// Preview a long press at position
    Longpress {
        x: f64,
        y: f64,
        #[arg(short, long)]
        output: Option<String>,
    },
    /// Preview a drag path from start to end
    Drag {
        x1: f64,
        y1: f64,
        x2: f64,
        y2: f64,
        #[arg(short, long)]
        output: Option<String>,
    },
    /// Preview a scroll action
    #[command(allow_negative_numbers = true)]
    Scroll {
        /// Horizontal scroll direction
        dx: i32,
        /// Vertical scroll direction
        dy: i32,
        /// X position
        #[arg(long)]
        x: Option<f64>,
        /// Y position
        #[arg(long)]
        y: Option<f64>,
        #[arg(short, long)]
        output: Option<String>,
    },
}
