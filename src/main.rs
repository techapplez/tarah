use std::path::Path;
use std::process::{exit, Command};
use std::env;
use colored::*;
use serde::Deserialize;
mod install;
mod cleanup;
mod update;
mod remove;
mod sync;

fn main() {
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

    let _cleanup = Command::new("rm")
        .args(&["-rf", cloned_pkgs_path.to_str().unwrap()])
        .status();

    let args: Vec<String> = env::args().collect();
    if args.len() == 3 && args[1] == "-S" {
        install::tarah_install_pkg(&args[2]);
    } else if args.len() == 3 && args[1] == "-R" {
        remove::tarah_remove_pkg(&args[2]);
    } else if args.len() > 1 && args[1] == "-U" {
        //do something
    } else if args.len() > 0 && args[1] == "-Sy" {
        sync::sync();
    } else {
        println!("Usage: tarah [-S package] [-R package] [-U package1 package2 ...]");
    }
}




fn usize_to_u64(value: usize) -> u64 {
    value as u64
} 


