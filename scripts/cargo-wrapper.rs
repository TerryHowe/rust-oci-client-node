use std::process::{exit, Command};

/// Tiny wrapper so that napi-rs (which uses Node `spawn()` — no shell, only real
/// executables work on Windows) can call `cargo auditable <args>` through the
/// `CARGO` env var.
fn main() {
    let status = Command::new("cargo")
        .arg("auditable")
        .args(std::env::args_os().skip(1))
        .status()
        .expect("failed to run cargo auditable");
    exit(status.code().unwrap_or(1));
}
