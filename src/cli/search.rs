use crate::helpers::rundebug;
use std::process::Command;

pub fn search(arg: &str, debug: bool) {
    let output = Command::new("pacman").args(["-Ss", arg]).output();
    println!("{:#?}", output)
}
