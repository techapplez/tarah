use colored::*;
use git2::Repository;
use serde::Deserialize;
use serde_json::from_str;
use std::env;
use std::error::Error;
use std::fs;
use std::io::BufRead;
use std::path::Path;
use std::process::{exit, Command};

#[derive(Deserialize, Debug, Clone)]
struct AurPackageInfo {
    results: Vec<AurPackage>,
}

#[derive(Deserialize, Debug, Clone)]
struct AurPackage {
    name: String,
    version: String,
    outdated: Option<bool>,
}


fn get_package_repo(package_name: &str) -> Result<String, Box<dyn Error>> {
    let output = Command::new("pacman")
        .args(&["-Qi", package_name])
        .output()?;

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let lines = stdout.lines();
        for line in lines {
            if line.contains("Repository") {
                let repo = line.split_whitespace().last().unwrap_or("unknown").to_string();
                return Ok(repo);
            }
        }
        return Ok("official".to_string());
    }

    let url = format!("https://aur.archlinux.org/rpc/v5/info?arg[]={}", package_name);
    let response = reqwest::blocking::get(&url)?.text()?;
    let package_info: AurPackageInfo = from_str(&response)?;

    if !package_info.results.is_empty() {
        return Ok("aur".to_string());
    }
    Ok("unknown".to_string())
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
        .args(&["-rf", cloned_pkgs_path.to_str().unwrap()])
        .status();

    let args: Vec<String> = env::args().collect();
    if args.len() == 3 && args[1] == "-S" {
        tarah_install_pkg(&args[2]);
    } else if args.len() == 3 && args[1] == "-R" {
        tarah_remove_pkg(&args[2]);
    } else if args.len() > 1 && args[1] == "-U" {
        let packages: Vec<String> = args.iter().skip(2).cloned().collect();
        tarah_update_all(packages);
    } else if args.len() == 1 {
        tarah_full_upgrade();
    } else {
        println!("Usage: tarah [-S package] [-R package] or no arguments for full system upgrade");
    }
}

fn cleanup() {
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
        eprintln!("Failed to delete tarah cache: {}", e);
    }
}

fn check_remote_git_repo(url: &str) -> bool {
    let output = Command::new("git")
        .args(&["ls-remote", "--exit-code", url])
        .status();

    match output {
        Ok(status) => status.success(),
        Err(e) => {
            eprintln!("Failed to execute git ls-remote: {}", e);
            false
        }
    }
}

fn tarah_remove_pkg(pack: &str) {
    let remove = Command::new("sudo")
        .args(&["pacman", "-R", pack])
        .status();

    match remove {
        Ok(status) => {
            if !status.success() {
                let code = status.code().unwrap_or(1);
                eprintln!("{}", &format!("Couldn't remove pkg {} because of exit code {}", pack, code).red().bold());
                cleanup();
                exit(1);
            }
        }
        Err(e) => {
            eprintln!("Failed to execute pacman: {}", e);
            cleanup();
            exit(1);
        }
    }
    cleanup();
}

fn usize_to_u64(value: usize) -> u64 {
    value as u64
}

fn tarah_install_pkg(pack: &str) {
    let pacman_test = Command::new("sudo")
        .args(&["pacman", "-S", pack])
        .status();


    if pacman_test.unwrap().success() {
       exit(0)
    }
    
    let default_url = "https://aur.archlinux.org/";
    let package_url = format!("{}{}.git", default_url, pack);
    
    if !check_remote_git_repo(package_url.as_str()) {
        eprintln!("{}", "repo duznt exist".red().bold());
        exit(1);
    } else {
        eprintln!("{}", "repo check passd".green());
    }

    let home = match env::var_os("HOME") {
        Some(h) => h,
        None => {
            eprintln!("{}", "Faild to get $HOME.".red().bold());
            exit(1);
        }
    };


    let clone_path = Path::new(&home)
        .join(".cache")
        .join("tarah")
        .join("git_cloney_thingy")
        .join(pack);
    let clone_path_str = clone_path.to_str().unwrap();

    let cloned_pkgs_path = Path::new(&home)
        .join(".cache")
        .join("tarah")
        .join("git_cloney_thingy");

    if let Err(e) = fs::create_dir_all(&clone_path) {
        eprintln!("Failed to create directory {:?}: {}", clone_path, e);
        exit(1);
    }

    let repo_result = Repository::clone(&package_url, &clone_path);

    match repo_result {
        Ok(_repo) => {
            println!("{}", &format!("cloning {} from {} to {}, please wait...", pack, package_url, clone_path_str).green());
        }
        Err(e) => {
            eprintln!("Failed to clone repository: {}", e);
            exit(1);
        }
    }

    let makepkg_status = Command::new("makepkg")
        .current_dir(&clone_path)
        .args(&["-si", "--noconfirm"])
        .status();

    match makepkg_status {
        Ok(status) => {
            if status.success() {
                println!("{}", "makepkg done installing".green());
            } else {
                eprintln!("{}", "makepkg did not want to install".red().bold());
                exit(1);
            }
        }
        Err(e) => {
            eprintln!("Failed to execute makepkg: {}", e);
            exit(1);
        }
    }

    let _cleanup = Command::new("rm")
        .args(&["-rf", cloned_pkgs_path.to_str().unwrap()]);
}

fn tarah_update_all(packages: Vec<String>) -> Result<(), Box<dyn Error>> {
    let url = format!("https://aur.archlinux.org/rpc/v5/info?arg[]={}", packages.join("&arg[]="));
    let response = reqwest::blocking::get(&url)?.text()?;
    let package_info: AurPackageInfo = from_str(&response)?;

    if package_info.results.is_empty() {
        println!("No packages found.");
        return Ok(());
    }

    for package in package_info.results {
        if package.outdated.unwrap_or(false) {
            println!("Updating package: {}", package.name);
            tarah_install_pkg(&package.name);
        } else {
            println!("Package {} is up to date.", package.name);
        }
    }

    Ok(())
}

fn tarah_full_upgrade(){
    let output = Command::new("sudo")
        .args(&["pacman"])
        .args(&["-Syu"])
        .args(&["--noconfirm"])
        .status();
    let g;
}