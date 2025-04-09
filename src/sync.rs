use std::process::{exit, Command as StdCommand};

pub fn sync() {
    let sync = StdCommand::new("pacman")
        .args(&["-Sy"])
        .output();
}