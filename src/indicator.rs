use std::process::Command;

/// Spawn a visual click indicator at the given screen position.
/// Runs asynchronously (fire-and-forget) so it won't block the action.
pub fn show_at(x: f64, y: f64) {
    if let Some(bin) = find_binary() {
        let _ = Command::new(bin)
            .args([x.to_string(), y.to_string()])
            .spawn();
    }
}

fn find_binary() -> Option<std::path::PathBuf> {
    // Look next to the current executable first
    if let Ok(exe) = std::env::current_exe() {
        if let Some(dir) = exe.parent() {
            let path = dir.join("aic-indicator");
            if path.exists() {
                return Some(path);
            }
        }
    }
    // Fallback: check PATH via `which`
    if let Ok(output) = Command::new("which").arg("aic-indicator").output() {
        if output.status.success() {
            let p = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !p.is_empty() {
                return Some(std::path::PathBuf::from(p));
            }
        }
    }
    None
}
