use std::process::Command as SysCommand;
use colored::Colorize;

pub fn update(debug: bool) {
    let pacman = SysCommand::new("pacman")
        .args(["-Qem"])
        .output()
        .unwrap_or_else(|_| panic!("{}", "Failed to run pacman".red()));

    println!("{pacman:?}")
}
