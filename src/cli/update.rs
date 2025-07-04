use crate::install::tarah_install_pkg;
use colored::Colorize;
use serde_json::Value;
use std::collections::HashMap;
use std::process::Command as SysCommand;

async fn get_aur_version(package_name: &str) -> Result<String, Box<dyn std::error::Error>> {
    let url = format!("https://aur.archlinux.org/rpc/v5/info/{}", package_name);

    let response = reqwest::get(&url).await?;
    let json: Value = response.json().await?;

    if let Some(results) = json["results"].as_array() {
        if let Some(first_result) = results.first() {
            if let Some(version) = first_result["Version"].as_str() {
                return Ok(version.to_string());
            }
        }
    }
    Err("Version nut fundz".into())
}

pub async fn update(debug: bool) {
    println!("-------pacman updat-------");

    let mut pacman_update = SysCommand::new("sudo")
        .args(["pacman", "-Syu"])
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .spawn()
        .expect("faild to update from pacman");

    /*let clear = SysCommand::new("clear")
        .stdout(std::process::Stdio::inherit())
        .spawn();
    let _ = clear.unwrap().wait_with_output(); */

    match pacman_update.wait() {
        Ok(status) => {
            if status.success() {
                println!("pacman updat sucess");
            } else {
                let code = status.code().unwrap_or(-1);
                println!("pacman updat faild with exit code: {}", code);
            }
        }
        Err(e) => {
            println!("faild to wait for pacman proces: {}", e);
            return;
        }
    }

    println!("--------aur updat--------");

    let pacman = SysCommand::new("pacman")
        .args(["-Qem"])
        .output()
        .unwrap_or_else(|_| panic!("{}", "Failed to run pacman".red()));

    let output = String::from_utf8(pacman.stdout)
        .unwrap_or_else(|_| panic!("{}", "Faild to makez magikz output".red()));

    let package_versions: HashMap<String, String> = output
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let mut parts = line.split_whitespace();
            let name = parts.next().unwrap_or_default().to_string();
            let version = parts.next().unwrap_or_default().to_string();
            (name, version)
        })
        .collect();

    let mut updates_available = false;

    for name in package_versions.keys() {
        match get_aur_version(name).await {
            Ok(aur_version) => {
                if aur_version != package_versions[name] {
                    updates_available = true;
                    tarah_install_pkg(&[name.to_owned()], debug);
                }
            }
            Err(e) => {
                println!("Faildz to get vershunz for packag {}: {}", name, e);
            }
        }
    }

    if !updates_available {
        println!("No AUR pakag updatt avail");
    }
}
