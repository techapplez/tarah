mod helpers;

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

fn run_if_debug(debug: bool, action: impl Fn()) {
    if debug {
        action();
    }
}


