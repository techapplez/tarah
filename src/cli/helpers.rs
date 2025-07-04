use colored::Colorize;

pub fn rundebug(debug: bool, mut action: impl FnMut(), mut elseaction: impl FnMut()) {
    if debug {
        println!("{}", "DEBUG BEGIN".yellow());
        action();
        println!("{}", "DEBUG END".yellow());
    } else {
        elseaction();
    }
}
