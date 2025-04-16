use std::env;
use std::path::Path;
use std::process::{exit, Command};
use colored::*;

pub fn cleanup() {
    let home = match env::var_os("HOME") {
        Some(home) => home,
        None => {
            eprintln!("{}", "Failed to get $HOME".red().bold());
            exit(1);
        }
    };

    let cloned_pkgs_path = Path::new(&home)
        .join(".cache")
        .join("tarah")
        .join("git_cloney_thingy");

    let status = Command::new("rm")
        .args(&["-rf", cloned_pkgs_path.to_str().unwrap()])
        .status();

    if let Err(e) = status {
        eprintln!("Faild to delete tarah cache: {}", e);
    }
}

//hello