use std::path::Path;
use std::process::{exit, Command};
use std::env;
use colored::*;
use crate::gradient::gout;

mod install;
mod cleanup;
mod update;
mod remove;
mod sync;
mod upgrade;
mod else_pacman;
mod search;
mod gradient;
mod helpers;

fn pacman_ops(operations: &str, debug: bool) {
    let operations = operations.split("???").map(|s| s.trim()).collect::<Vec<_>>();

    for operation in operations {
        if operation.is_empty() {
            continue;
        }

        let parts: Vec<&str> = operation.split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }

        match parts[0] {
            "-S" => {
                if parts.len() > 1 {
                    let packages = parts[1..].to_vec();
                    let packages: Vec<String> = packages.iter().map(|&s| s.to_string()).collect();
                    install::tarah_install_pkg(&packages, debug);
                    println!("-------------------------------------------------------------")
                }
            },
            "-R" => {
                if parts.len() > 1 {
                    remove::tarah_remove_pkg(parts[1], debug);
                    println!("-------------------------------------------------------------")
                }
            },
            "-Sy" => {
                sync::sync(debug);
                if parts.len() > 1 {
                    sync::supd(parts[1], debug);
                    println!("-------------------------------------------------------------")
                }
            },
            "-Syu" => {
                update::update(debug);
                println!("-------------------------------------------------------------")
            },
            "-U" => {
                if parts.len() > 1 {
                    upgrade::upgrade(parts[1], debug);
                    println!("-------------------------------------------------------------")
                }
            },
            "-C" => {
                cleanup::cleanup(debug);
                println!("-------------------------------------------------------------")
            },
            "-test" => {
                println!("This is a test, lol");
                println!("-------------------------------------------------------------")
            },
            _ => {
                if parts[0].starts_with('-') {
                    else_pacman::else_pacman(operation, debug);
                    println!("-------------------------------------------------------------")
                } else {
                    println!("{}", format!("Unknown operation: {}", parts[0]).red());
                }
            }
        }
    }
}

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
        .args(["-rf", cloned_pkgs_path.to_str().unwrap()])
        .status();

    let args: Vec<String> = env::args().collect();

    let mut dbg_state = false;
    let mut cmd_index = 1;

    if args.len() > 1 && (args[1] == "--debug" || args[1] == "-dbg") {
        dbg_state = true;
        cmd_index = 2;
    }

    gout(" _                  _
| |_ __ _ _ __ __ _| |__
| __/ _` | '__/ _` | '_ \\
| || (_| | | | (_| | | | |
 \\__\\__,_|_|  \\__,_|_| |_|", dbg_state);

    if args.len() <= cmd_index {
        update::update(dbg_state);
        return;
    }

    let operations = args[cmd_index..].join(" ");
    pacman_ops(&operations, dbg_state);
}