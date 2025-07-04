use colored::*;
use gradient::gout;
use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::path::Path;
use std::process::{Command, exit};

mod cleanup;
mod else_pacman;
mod gradient;
mod helpers;
mod install;
mod remove;
mod search;
mod sync;
mod update;
mod upgrade;

type MyResult<T> = Result<T, Box<dyn Error>>;

async fn pacman_ops(operations: &str, debug: bool) -> MyResult<()> {
    let operations = operations
        .split("???")
        .map(|s| s.trim())
        .collect::<Vec<_>>();

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
            }
            "-R" => {
                if parts.len() > 1 {
                    remove::tarah_remove_pkg(parts[1], debug);
                    println!("-------------------------------------------------------------")
                }
            }
            "-Sy" => {
                sync::sync(debug);
                if parts.len() > 1 {
                    sync::supd(parts[1], debug);
                    println!("-------------------------------------------------------------")
                }
            }
            "-Syu" => {
                update::update(debug).await;
                println!("-------------------------------------------------------------")
            }
            "-U" => {
                if parts.len() > 1 {
                    upgrade::upgrade(parts[1], debug);
                    println!("-------------------------------------------------------------")
                }
            }
            "-C" => {
                cleanup::cleanup(debug);
                println!("-------------------------------------------------------------")
            }
            "-test" => {
                println!("This is a test, lol");
                println!("-------------------------------------------------------------")
            }
            "--cleanup" => {
                cleanup::cleanup(debug);
                println!("-------------------------------------------------------------")
            }
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
    Ok(())
}

#[tokio::main]
async fn main() {
    let home: OsString = if let Some(home) = env::var_os("HOME") {
        home
    } else if let None = env::var_os("HOME") {
        eprintln!("{}", "Failed to get $HOME".red().bold());
        exit(1);
    } else {
        unreachable!()
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

    gout(
        " _                  _
| |_ __ _ _ __ __ _| |__
| __/ _` | '__/ _` | '_ \\
| || (_| | | | (_| | | | |
 \\__\\__,_|_|  \\__,_|_| |_|",
        dbg_state,
    );

    if args.len() <= cmd_index {
        update::update(dbg_state).await;
        return;
    }

    let operations = args[cmd_index..].join(" ");
    pacman_ops(&operations, dbg_state)
        .await
        .expect("Failed to run pacman operations")
}
