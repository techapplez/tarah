use std::process::Command;

pub fn else_pacman(arg: &str, debug: bool) {
    let pacman_another_fucking_useless_test_because_i_have_no_idea_how_to_do_this = Command::new("pacman")
        .args(&["{}", arg])
        .status();
    
    if pacman_another_fucking_useless_test_because_i_have_no_idea_how_to_do_this.unwrap().success() {
        println!("Sucessssssss")
    } else {
        println!("Faild, awwwwwww")
    }
}