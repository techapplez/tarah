use std::fs;
use std::process::Command;
pub fn tarah_update() {
    let output = Command::new("sudo")
        .args(&["pacman"])
        .args(&["-Syu"])
        .args(&["--noconfirm"])
        .status();

    if !output.unwrap().success() {
        //oioioioiaaoiaoiao
    } else {
        //oaioaioaiaoioioaiao
    }

    let _op = Command::new("sh")
        .args(&["-c", "sudo pacman -Qmi | jc --pacman > /tmp/aur.tarah.json"])
        .status();

    let json = fs::read_to_string("/tmp/aur.tarah.json").unwrap();
    println!("{}", json);
}

//hello