use colored::Colorize;

#[macro_export]
macro_rules! debug_exec {
    ($debug_flag:expr, $color:expr, $code:block) => {
        if $debug_flag == true {
            use colored::*;
            println!("DEBUG BEGIN".$color);
            $code
            println!("DEBUG END".$color);
        }
    };
}

pub fn rundebug(debug: bool, mut action: impl FnMut()) {
    if debug {
        println!("{}", "DEBUG BEGIN".yellow());
        action();
        println!("{}", "DEBUG END".yellow());
    }
}
