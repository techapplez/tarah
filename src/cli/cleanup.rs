use crate::helpers::rundebug;
use colored::*;
use std::env;
use std::fs;
use std::path::Path;
use std::process::{Command, exit};

pub(crate) fn cleanup(debug: bool) {
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

    if !cloned_pkgs_path.exists() {
        if let Err(e) = fs::create_dir_all(&cloned_pkgs_path) {
            eprintln!("Failed to create cache directory: {}", e);
            return;
        }
        println!("{}", "Created cache directory".green());
    }

    rundebug(
        debug,
        || match fs::read_dir(&cloned_pkgs_path) {
            Ok(entries) => {
                let mut files = Vec::new();
                for entry in entries {
                    if let Ok(entry) = entry {
                        files.push(entry.path());
                    }
                }
                if files.is_empty() {
                } else {
                    println!(
                        "Found {} items to delete in: {}",
                        files.len(),
                        cloned_pkgs_path.display()
                    );
                    println!("Contents: {:#?}", files);
                }
            }
            Err(e) => {
                println!("Could not read directory contents: {}", e);
            }
        },
        || {
            println!("Cleaning up cache directory contents...");
        },
    );

    let cache_glob = format!("{}/*", cloned_pkgs_path.to_str().unwrap());
    let status = Command::new("rm").args(["-rf"]).arg(&cache_glob).status();

    match status {
        Ok(exit_status) if exit_status.success() => {
            println!("{}", "Cache contents cleaned successfully".green());
        }
        Ok(_) => {
            println!("{}", "Cache directory is now empty".yellow());
        }
        Err(e) => {
            eprintln!("Failed to execute cleanup command: {}", e);
        }
    }
}
