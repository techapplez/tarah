use std::process::Command as SysCommand;
use colored::Colorize;

pub fn update() {
    let pacman = SysCommand::new("pacman")
        .args(&["-Qem"])
        .output()
        .expect(&format!("{}", "Failed to run pacman".red()));

    
}