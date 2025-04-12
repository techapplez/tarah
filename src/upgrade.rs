use colored::*;
use std::process::Command;

pub fn upgrade(pack: &str) {
    let process = Command::new("pacman").args(&["-U", &pack]).status();
    if process.expect("").success() {
        println!("{}", format!("{}", "Upgraded pkg".green()));
    } else {
        println!("{}", format!("{}", "Upgrade faild".red()));
    }
}
