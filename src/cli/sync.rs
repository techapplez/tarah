use std::process::{exit, Command};
use std::{env, fs};
use std::path::Path;
use colored::*;
use git2::Repository;
use crate::install::check_remote_git_repo;

pub fn sync(debug: bool) {
    let output = Command::new("sudo")
        .args(&["pacman"])
        .args(&["-Sy"])
        .status();

    if output.unwrap().success() {
        println!("{}", format!("{}" ,"Synced lol".green()))
    } else {
        eprintln!("{}", format!("{}", "Didnt sync aww".red().bold()))
    }
}

pub fn supd(pack: &str, debug: bool) {
    
    let clean_pkg = format!("^{} ", pack);

    let pacman_test = Command::new("pacman")
        .args(&["-Ss", clean_pkg.as_str()])
        .output();

    match pacman_test {
        Ok(output) => {
            if !output.stdout.is_empty() {
                let _pacman_run = Command::new("sudo")
                    .args(&["pacman", "-S", pack])
                    .status();
            } else {

                let default_url = "https://aur.archlinux.org/";
                let package_url = format!("{}{}.git", default_url, pack);
                if !check_remote_git_repo(package_url.as_str()).expect("") {
                    eprintln!("{}", "repo duznt exist".red().bold());
                    exit(1);
                } else {
                    eprintln!("{}", "repo check passd".green());

                    let home = env::var_os("HOME").unwrap_or_else(|| {
                        eprintln!("{}", "Faild to get $HOME.".red().bold());
                        exit(1);
                    });

                    let clone_path = Path::new(&home)
                        .join(".cache")
                        .join("tarah")
                        .join("git_cloney_thingy")
                        .join(pack);

                    if fs::create_dir_all(&clone_path).is_err() {
                        eprintln!("Failed to create directory {:?}", clone_path);
                        exit(1);
                    }

                    match Repository::clone(&package_url, &clone_path) {
                     Ok(_) => println!("{}", format!("cloninn {} from {} to {}", pack, package_url, clone_path.to_str().unwrap()).green()),
                        Err(e) => {
                            eprintln!("Failed to clon repo: {}", e);
                            exit(1);
                        }
                    }

                    match Command::new("makepkg")
                        .current_dir(&clone_path)
                        .args(&["-si", "--noconfirm"])
                        .status()
                    {
                        Ok(status) if status.success() => println!("{}", "makepkg done installing".green()),
                        _ => {
                            eprintln!("{}", "makepkg did not want to install".red().bold());
                            exit(1);
                        }
                    }

                    if Command::new("rm")
                        .args(&["-rf", clone_path.to_str().unwrap()])
                        .status()
                        .is_err()
                    {
                        eprintln!("Faild cleenin up.");
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("pacman said: {}", e);
            exit(1);
        }
    }
}

//hello