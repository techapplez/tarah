use std::env::args;
use std::io;
use std::path::Path;
use git2::Repository;
use std::process::{exit, Command};

fn main() {
    let mut pack = String::new();

    println!("Wat pkg u want? ");

    io::stdin()
        .read_line(&mut pack)
        .expect("wtf u doin");

    let pack = pack.trim();
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

    let repo = Repository::clone(&package_url, &clone_path)
        .expect(&format!("Something went wrong trying to clone {}. Either you are dumb or git fucked up.", package_url));

    println!("Cloned {} from {} to {}.", pack, package_url, clone_path.display());

    let op = Command::new("makepkg")
        .current_dir(&clone_path)
        .args(&["-si"])
        .status()
        .expect("Failed to execute makepkg");

    let cleanup = Command::new("rm")
        .args(&["-rf", cloned_pkgs_path.to_str().unwrap()])
        .status()
        .expect("makepkg no want make package");

    exit(0)
}