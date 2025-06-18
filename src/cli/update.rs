use colored::Colorize;
use std::process::Command as SysCommand;
use std::collections::HashMap;
use reqwest;
use serde_json::Value;
use tokio;
use crate::helpers::rundebug;
use crate::install::tarah_install_pkg;
use crate::remove::tarah_remove_pkg;

async fn get_aur_version(package_name: &str) -> Result<String, Box<dyn std::error::Error>> {
    let url = format!(
        "https://aur.archlinux.org/rpc/v5/info/{}",
        package_name
    );

    let response = reqwest::get(&url).await?;
    let json: Value = response.json().await?;

    let version = json["results"][0]["Version"]
        .as_str()
        .ok_or("Version not found")?
        .to_string();

    Ok(version)
}

pub async fn update(debug: bool) {
    let pacman = SysCommand::new("pacman")
        .args(["-Qem"])
        .output()
        .unwrap_or_else(|_| panic!("{}", "Failed to run pacman".red()));

    let output = String::from_utf8(pacman.stdout)
        .unwrap_or_else(|_| panic!("{}", "Failed to convert output to string".red()));

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

    for name in package_versions.keys() {
        if let Ok(aur_version) = get_aur_version(name).await {
            if aur_version != package_versions[name] {
                let mut state = false;
                rundebug(debug, || {
                    tarah_install_pkg(&[name.to_owned()], debug);
                    state = true;
                });
                if !state {
                    tarah_install_pkg(&[name.to_owned()], state);
                }
            }
        } else {
            println!("No AUR package updates available")
        }
    }
}