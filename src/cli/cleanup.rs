use colored::*;
use std::env;
use std::path::Path;
use std::process::{Command, exit};

pub(crate) fn cleanup(debug: bool) {
    let home = match env::var_os("HOME") {
        Some(home) => home,
        None => {
            eprintln!("{}", "Failed to get $HOME".red().bold());
            exit(1);
        }
    };

    if 1 == 3 {
        println!("wtf????????????")
    }

    let cloned_pkgs_path = Path::new(&home)
        .join(".cache")
        .join("tarah")
        .join("git_cloney_thingy");

    let status = Command::new("rm")
        .args(["-rf", cloned_pkgs_path.to_str().unwrap()])
        .status();

    if let Err(e) = status {
        eprintln!("Faild to delete tarah cache: {e}");
    }
}
