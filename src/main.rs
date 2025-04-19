use std::path::Path;
use std::process::{exit, Command};
use std::env;
use colored::*;

mod install;
mod cleanup;
mod update;
mod remove;
mod sync;
mod upgrade;
mod else_pacman;


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

    // Modified package installation handling
    if args.len() >= 3 && args[1] == "-S" {
        let packages = args[2..].to_vec();
        install::tarah_install_pkg(&packages);
    }
    else if args.len() == 3 && args[1] == "-R" {
        remove::tarah_remove_pkg(&args[2]);
    }
    else if args.len() == 3 && args[1] == "-U" {
        upgrade::upgrade(&args[2])
    }
        
    else if args[1] == "-Syu" {
        //update::tarah_update()
    }
        
    else if args[1] == "-Sy" {
        if args.len() > 2 {
            sync::sync();
            sync::supd(&args[2])
        } else {
            sync::sync();
        }
    }
    else if args.len() == 3 && args[1] == "-C" {
        cleanup::cleanup();
    }
    else if args[1] == "-test" {
        println!("This is a test, {}", "lol");
    }
    else {
        for arg in args {
            if arg.starts_with("-") || arg.starts_with("--") {
            } else if arg.is_empty() {
                // There will be an update function here
                //update::tarah_update()
            }
            
            else {
                //search
            }
        }
    }
}