mod cli;
mod error;
mod indicator;
mod keyboard;
mod keymap;
mod mouse;
mod preview;
mod screenshot;

use clap::Parser;
use cli::{Cli, Command, KeyAction, MouseAction, PreviewAction};

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Command::Key { action } => match action {
            KeyAction::Press { key } => keyboard::press_key(&key),
            KeyAction::Combo { keys } => keyboard::key_combo(&keys),
            KeyAction::Down { key } => keyboard::key_down(&key),
            KeyAction::Up { key } => keyboard::key_up(&key),
        },
        Command::Type { text, delay_ms } => keyboard::type_text(&text, delay_ms),
        Command::Mouse { action } => match action {
            MouseAction::Move { x, y } => mouse::move_to(x, y),
            MouseAction::Click { x, y } => mouse::click(x, y),
            MouseAction::Doubleclick { x, y } => mouse::double_click(x, y),
            MouseAction::Rightclick { x, y } => mouse::right_click(x, y),
            MouseAction::Longpress {
                x,
                y,
                duration_ms,
            } => mouse::long_press(x, y, duration_ms),
            MouseAction::Drag {
                x1,
                y1,
                x2,
                y2,
                duration_ms,
            } => mouse::drag(x1, y1, x2, y2, duration_ms),
            MouseAction::Scroll { dx, dy, x, y } => {
                let at = match (x, y) {
                    (Some(x), Some(y)) => Some((x, y)),
                    _ => None,
                };
                mouse::scroll(dx, dy, at)
            }
            MouseAction::Preview { action } => {
                let output = match &action {
                    PreviewAction::Click { output, .. }
                    | PreviewAction::Doubleclick { output, .. }
                    | PreviewAction::Rightclick { output, .. }
                    | PreviewAction::Move { output, .. }
                    | PreviewAction::Longpress { output, .. }
                    | PreviewAction::Drag { output, .. }
                    | PreviewAction::Scroll { output, .. } => output.as_deref(),
                };
                preview::preview_mouse_action(&action, output)
            }
        },
        Command::Screenshot { output, base64 } => {
            screenshot::take_screenshot(output.as_deref(), base64)
        }
    };

    if let Err(e) = result {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}
