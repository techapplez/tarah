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
    
    if args.len() == 3 && args[1] == "-S" {
        install::tarah_install_pkg(&args[2]);
    } 
    
    else if args.len() == 3 && args[1] == "-R" {
        remove::tarah_remove_pkg(&args[2]);
    } 
    
    else if args.len() == 3 && args[1] == "-U" {
        upgrade::upgrade(&args[2])
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
        
    else if args[1] == "-Fuck" {
        //Do Some() thing, Ok()?
    }

    else {
        else_pacman::else_pacman(&args[2]);
    }
}


//I like Rust!
