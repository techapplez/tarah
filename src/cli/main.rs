use clap::{Parser, Subcommand};
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

#[derive(Parser)]
#[command(name = "tarah")]
#[command(about = "AUR package manager")]
#[command(long_about = None)]
struct Cli {
    #[arg(long, short = 'd', alias = "dbg")]
    debug: bool,

    #[command(subcommand)]
    command: Option<Commands>,

    #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
    raw_args: Vec<String>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "install", short_flag = 'S')]
    Install {
        packages: Vec<String>,
    },
    #[command(name = "remove", short_flag = 'R')]
    Remove {
        package: String,
    },
    #[command(name = "sync")]
    Sync {
        package: Option<String>,
    },
    #[command(name = "update")]
    Update,
    #[command(name = "upgrade", short_flag = 'U')]
    Upgrade {
        package: String,
    },
    #[command(name = "cleanup", short_flag = 'C')]
    Cleanup,
    #[command(name = "test")]
    Test,
    #[command(name = "search")]
    Search {
        query: String,
    },
}

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
                    search::search(operation, debug);
                }
            }
        }
    }
    Ok(())
}

fn parse_raw_args_for_debug_and_commands(args: &[String]) -> (bool, Vec<String>) {
    let mut debug = false;
    let mut filtered_args = Vec::new();

    for arg in args {
        if arg == "--debug" || arg == "-dbg" {
            debug = true;
        } else {
            filtered_args.push(arg.clone());
        }
    }

    (debug, filtered_args)
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

    let cli = Cli::parse();
    let mut debug = cli.debug;

    gout(
        " _                  _
| |_ __ _ _ __ __ _| |__
| __/ _` | '__/ _` | '_ \\
| || (_| | | | (_| | | | |
 \\__\\__,_|_|  \\__,_|_| |_|",
        debug,
    );

    match cli.command {
        Some(Commands::Install { packages }) => {
            install::tarah_install_pkg(&packages, debug);
            println!("-------------------------------------------------------------")
        }
        Some(Commands::Remove { package }) => {
            remove::tarah_remove_pkg(&package, debug);
            println!("-------------------------------------------------------------")
        }
        Some(Commands::Sync { package }) => {
            sync::sync(debug);
            if let Some(pkg) = package {
                sync::supd(&pkg, debug);
            }
            println!("-------------------------------------------------------------")
        }
        Some(Commands::Update) => {
            update::update(debug).await;
            println!("-------------------------------------------------------------")
        }
        Some(Commands::Upgrade { package }) => {
            upgrade::upgrade(&package, debug);
            println!("-------------------------------------------------------------")
        }
        Some(Commands::Cleanup) => {
            cleanup::cleanup(debug);
            println!("-------------------------------------------------------------")
        }
        Some(Commands::Test) => {
            println!("This is a test, lol");
            println!("-------------------------------------------------------------")
        }
        Some(Commands::Search { query }) => {
            search::search(&query, debug);
        }
        None => {
            if cli.raw_args.is_empty() {
                update::update(debug).await;
                return;
            }

            let (raw_debug, filtered_args) = parse_raw_args_for_debug_and_commands(&cli.raw_args);
            debug = debug || raw_debug;

            let operations = filtered_args.join(" ");
            pacman_ops(&operations, debug)
                .await
                .expect("Failed to run pacman operations")
        }
    }
}
