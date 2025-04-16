use std::process::{exit, Command};
use colored::Colorize;
use crate::cleanup;

pub fn tarah_remove_pkg(pack: &str) {
    let remove = Command::new("sudo")
        .args(&["pacman", "-R", pack])
        .status();

    match remove {
        Ok(status) => {
            if !status.success() {
                let code = status.code().unwrap_or(1);
                eprintln!("{}", &format!("Couldn't remove pkg {} because of exit code {}", pack, code).red().bold());
                cleanup::cleanup();
                exit(1);
            }
        }
        Err(e) => {
            eprintln!("Failed to execute pacman: {}", e);
            cleanup::cleanup();
            exit(1);
        }
    }
    cleanup::cleanup();
}

//hello