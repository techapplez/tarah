#[macro_export]
macro_rules! debug_exec {
    ($debug_flag:expr, $code:block) => {
        if $debug_flag {
            $code
        }
    };
}


