use std::process::{Command, exit, Stdio};
use colored::*;

pub fn upgrade(pack: &str) {
    let mut process = Command::new("pacman")
        .args(&["-U", &pack])
        .status();
    if process.expect("").success() {
        println!("{}", format!("{}", "Upgraded pkg".green()));
    } else {
        println!("{}", format!("{}", "Upgrade faild".red()));
    }
}
