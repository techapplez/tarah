use std::io::{self, Write};
use reqwest::blocking::get;
use serde_json::{Value, from_str};
use std::path::Path;
use git2::Repository;
use std::process::{Command, Stdio};
use std::env;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

struct AurPackage {
    name: String,
    version: String,
}

fn get_installed_aur_packages() -> io::Result<Vec<AurPackage>> {
    let output = Command::new("pacman")
        .args(&["-Qm"])
        .output()?;

    if !output.status.success() {
        return Err(io::Error::new(io::ErrorKind::Other,
                                  "Failed to get installed AUR packages"));
    }

    let packages = String::from_utf8_lossy(&output.stdout)
        .lines()
        .filter_map(|line| {
            let mut parts = line.split_whitespace();
            Some(AurPackage {
                name: parts.next()?.to_string(),
                version: parts.next()?.to_string(),
            })
        })
        .collect();

    Ok(packages)
}

fn main() {

    let home = std::env::var_os("HOME").expect("Failed to get HOME directory");
    let cloned_pkgs_path = Path::new(&home)
        .join(".cache")
        .join("tarah")
        .join("git_cloney_thingy");

    let _cleanup = Command::new("rm")
        .args(&["-rf", cloned_pkgs_path.to_str().unwrap()])
        .status()
        .expect("PermissionError, probably");

    let args: Vec<String> = env::args().collect();
    if args.len() == 3 && args[1] == "-S" {
        tarah_install_pkg(&args[2]);
    } else {
        println!("Usage: ./tarah -S <package name>");
    }
}



fn tarah_install_pkg(pack: &str) {

    let default_url = "https://aur.archlinux.org/";
    let package_url = format!("{}{}.git", default_url, pack);
    let home = std::env::var_os("HOME").expect("Failed to get HOME directory");
    let clone_path = Path::new(&home)
        .join(".cache")
        .join("tarah")
        .join("git_cloney_thingy")
        .join(pack);

    let cloned_pkgs_path = Path::new(&home)
        .join(".cache")
        .join("tarah")
        .join("git_cloney_thingy");

    std::fs::create_dir_all(&clone_path).expect("Failed to create directories");

    let _repo = Repository::clone(&package_url, &clone_path)
        .expect(&format!("Something went wrong trying to clone {}. Either you are dumb or git fucked up.", package_url));

    println!("Cloned {} from {} to {}.", pack, package_url, clone_path.display());

    let _op = Command::new("makepkg")
        .current_dir(&clone_path)
        .args(&["-si"])
        .status()
        .expect("Failed to execute makepkg");

    let _cleanup = Command::new("rm")
        .args(&["-rf", cloned_pkgs_path.to_str().unwrap()])
        .status()
        .expect("PermissionError, probably");
}

// ------------------------------------------------------------------------


