use std::process::{Command as SysCommand, Stdio};
use colored::Colorize;

pub fn update(debug: bool) {
    let pacman = SysCommand::new("pacman")
        .args(&["-Qem"])
        .output()
        .expect(&format!("{}", "Failed to run pacman".red()));

    println!("{:?}", pacman)
}
