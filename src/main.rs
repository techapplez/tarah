use std::io::{self, Write};
use serde_json::Value;
use std::path::Path;
use git2::Repository;
use std::process::Command;
use std::env;
use reqwest::blocking;
use std::fs;
use std::collections::HashMap;

fn updatinnn() -> Result<(), Box<dyn std::error::Error>> {
    let output = Command::new("pacman").args(&["-Qm"]).output()?;
    let aur_packages: HashMap<_, _> = String::from_utf8(output.stdout)?
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            (parts[0].to_string(), parts[1].to_string())
        })
        .collect();

    let packages_str = aur_packages.keys().cloned().collect::<Vec<String>>().join("&arg[]=");
    let url = format!("https://aur.archlinux.org/rpc/?v=5&type=info&arg[]={}", packages_str);

    let response: Value = blocking::get(&url)?.json()?;

    let mut packages_to_update = Vec::new();
    if let Some(results) = response["results"].as_array() {
        for pkg_info in results {
            let name = pkg_info["Name"].as_str().unwrap();
            let aur_version = pkg_info["Version"].as_str().unwrap();
            if let Some(installed_version) = aur_packages.get(name) {
                if installed_version != aur_version {
                    packages_to_update.push(name.to_string());
                }
            }
        }
    }

    if !packages_to_update.is_empty() {
        println!("New versions available for: ");
        for package in &packages_to_update {
            println!("  {}", package);
        }

        print!("Do you want to perform a full system upgrade? (y/n): ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        if input.trim().eq_ignore_ascii_case("y") {
            println!("Performing full system upgrade...");
            let status = Command::new("sudo")
                .args(&["pacman", "-Syu", "--noconfirm"])
                .status()?;

            if !status.success() {
                return Err("Failed to perform system upgrade".into());
            }

            println!("Updating AUR packages...");
            for package in packages_to_update {
                let home = dirs::home_dir().expect("Failed to get home directory");
                let clone_path = home.join(".cache").join("tarah").join("git_cloney_thingy").join(&package);
                let repo_url = format!("https://aur.archlinux.org/{}.git", package);

                println!("Updating {}...", package);

                fs::create_dir_all(&clone_path)?;
                Repository::clone(&repo_url, &clone_path)?;

                Command::new("makepkg")
                    .current_dir(&clone_path)
                    .args(&["-si", "--noconfirm"])
                    .status()?;

                fs::remove_dir_all(&clone_path)?;
            }
        } else {
            println!("No updates were performed.");
        }
    } else {
        println!("All AUR packages are up to date.");
    }

    Ok(())
}

fn tarah_install_pkg(pack: &str) -> Result<(), Box<dyn std::error::Error>> {
    let repo_url = format!("https://aur.archlinux.org/{}.git", pack);
    let home = dirs::home_dir().expect("Failed to get home directory");
    let clone_path = home.join(".cache").join("tarah").join("git_cloney_thingy").join(pack);

    fs::create_dir_all(&clone_path.parent().unwrap())?;

    Repository::clone(&repo_url, &clone_path)?;
    Command::new("makepkg")
        .current_dir(&clone_path)
        .args(&["-si", "--noconfirm"])
        .status()?;

    fs::remove_dir_all(&clone_path)?;
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    let home = dirs::home_dir().expect("Failed to get home directory");
    let cloned_pkgs_path = home.join(".cache").join("tarah").join("git_cloney_thingy");
    if cloned_pkgs_path.exists() {
        fs::remove_dir_all(&cloned_pkgs_path)?;
    }

    match args.len() {
        1 => updatinnn(),
        3 if args[1] == "-S" => tarah_install_pkg(&args[2]),
        _ => {
            eprintln!("Usage: {} [-S <package>]", args[0]);
            std::process::exit(1);
        }
    }
}
