use std::{env, fs, io};
use std::path::PathBuf;
use std::process::{Command, Stdio};
use colored::*;
use git2::Repository;
use rayon::prelude::*; 

pub fn check_remote_git_repo(url: &str) -> io::Result<bool> {
    let status = Command::new("git")
        .args(&["ls-remote", "--exit-code", url])
        .status()?;

    Ok(status.success())
}

#[derive(Debug, Clone)]
enum PackageSource {
    Official(String),
    Aur(String),
    NotFound(String),
}
pub fn tarah_install_pkg(packs: &[String], debug: bool) {

    let results: Vec<Result<PackageSource, (String, String)>> = packs 
        .par_iter() 
        .map(|pack| {
            let pacman_check = Command::new("pacman")
                .args(&["-Ss", &format!("^{}$", pack)]) 
                .output();

            match pacman_check {
                Ok(output) => {
                    if !output.stdout.is_empty() {
                        Ok(PackageSource::Official(pack.clone()))
                    } else {
                        let url = format!("https://aur.archlinux.org/{}.git", pack);
                        match check_remote_git_repo(&url) { 
                            Ok(true) => Ok(PackageSource::Aur(pack.clone())),
                            Ok(false) => Ok(PackageSource::NotFound(pack.clone())),
                            Err(e) => {
                                let err_msg = format!("Err checkinnnn AUR for {}: {}", pack, e);
                                eprintln!("Error: {}", err_msg); 
                                Err((pack.clone(), err_msg)) 
                            }
                        }
                    }
                }
                Err(e) => {
                    let err_msg = format!("Error checking official repos for {}: {}", pack, e);
                    eprintln!("Error: {}", err_msg); 
                    Err((pack.clone(), err_msg))
                }
            }
        })
        .collect();
    
    let mut printuse_pacman: Vec<String> = Vec::new(); 
    let mut printuse_aur: Vec<String> = Vec::new(); 
    let mut not_found_pkgs: Vec<String> = Vec::new();
    let mut check_errors: Vec<(String, String)> = Vec::new();

    for result in results {
        match result {
            Ok(PackageSource::Official(pkg)) => printuse_pacman.push(pkg),
            Ok(PackageSource::Aur(pkg)) => printuse_aur.push(pkg),
            Ok(PackageSource::NotFound(pkg)) => not_found_pkgs.push(pkg),
            Err(e) => check_errors.push(e), // Collect errors instead of exiting
        }
    }

    for pack in &not_found_pkgs {
        println!("Package {} found nowhere :(", pack);
    }
    
    println!("Sync explicit: {:?}", printuse_pacman);
    println!("AUR explicit: {:?}", printuse_aur);


    if !printuse_pacman.is_empty() {
        let mut pacman_args = vec!["-S", "--noconfirm"];
        let pkgs_str: Vec<&str> = printuse_pacman.iter().map(AsRef::as_ref).collect();
        pacman_args.extend(pkgs_str);

        let status = Command::new("sudo")
            .arg("pacman")
            .args(&pacman_args)
            .status();

        match status {
            Ok(s) if s.success() => {
                for pack in &printuse_pacman {
                }
            }
            Ok(s) => {
            }
            Err(e) => {
            }
        }
    }

    if !printuse_aur.is_empty() {
        let home = match env::var_os("HOME") {
            Some(h) => PathBuf::from(h),
            None => {
                eprintln!("{}", "Faild to get $HOME.".red().bold());
                return;
            }
        };
        let base_build_dir = home
            .join(".cache")
            .join("tarah")
            .join("git_cloney_thingy");

        if let Err(e) = fs::create_dir_all(&base_build_dir) {
            return;
        }
        
        let aur_install_results: Vec<Result<String, (String, String)>> = printuse_aur
            .par_iter()
            .map(|pack| {
                let package_url = format!("https://aur.archlinux.org/{}.git", pack);
                let clone_path = base_build_dir.join(pack);

                println!(
                    "{}",
                    format!(
                        "[{}] cloninn {} from {} to {}", 
                        pack,
                        pack, 
                        package_url,
                        clone_path.to_str().unwrap_or("INVALID_PATH") 
                    )
                        .green()
                );


                if clone_path.exists() {
                    if let Err(e) = fs::remove_dir_all(&clone_path) {
                    }
                }
                if let Err(e) = fs::create_dir_all(&clone_path) {
                    let err_msg = format!("Failed to create directory {:?}: {}", clone_path, e);
                    eprintln!("{}", format!("[{}] Error: {}", pack, err_msg).red()); 
                    return Err((pack.clone(), err_msg));
                }


                match Repository::clone(&package_url, &clone_path) {
                    Ok(_) => {
                    }
                    Err(e) => {
                        let err_msg = format!("Failed to clon repo: {}", e);
                        eprintln!("{}", format!("[{}] {}", pack, err_msg).red()); 
                        return Err((pack.clone(), err_msg));
                    }
                }

                let makepkg_status = Command::new("makepkg")
                    .current_dir(&clone_path) 
                    .args(&["-si", "--noconfirm"])
                    .stderr(Stdio::piped())
                    .status();

                match makepkg_status {
                    Ok(status) if status.success() => {
                        println!("{}", format!("[{}] {}", pack, "makepkg done installing".green()));

                        if Command::new("rm")
                            .args(&["-rf", clone_path.to_str().unwrap_or("")]) 
                            .status()
                            .is_err()
                        {
                            eprintln!("{}", format!("[{}] {}", pack, "Faild cleenin up.").yellow());
                        }
                        Ok(pack.clone())
                    }
                    Ok(_status) => {
                        let err_msg = "makepkg did not want to install".to_string();
                        eprintln!("{}", format!("[{}] {}", pack, err_msg.red().bold()));
                        let output = Command::new("makepkg")
                            .current_dir(&clone_path)
                            .args(&["-si", "--noconfirm"])
                            .output();
                        if let Ok(out) = output {
                            if !out.stderr.is_empty() {
                                eprintln!("{}", String::from_utf8_lossy(&out.stderr).trim().red());
                            }
                        }
                        Err((pack.clone(), err_msg))
                    }
                    Err(e) => {
                        let err_msg = format!(" ");
                        eprintln!();
                        Err((pack.clone(), err_msg))
                    }
                }
            })
            .collect();
        
        let mut success_count = 0;
        let mut fail_count = 0;
        for result in aur_install_results {
            match result {
                Ok(pkg) => {
                    success_count += 1;
                }
                Err((pkg, _err)) => {
                    fail_count += 1;
                }
            }
        }
    }
}

//hello there
