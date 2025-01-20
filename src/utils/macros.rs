#[macro_export]
macro_rules! assert_eq_with_cleanup {
    ($left: expr, $right: expr, $path: expr) => {
        if $left != $right {
            test_utils::remove_config_from_path($path);
            panic!("Assertion failed: {:?} != {:?}", $left, $right);
        }
    };
}

#[macro_export]
macro_rules! assert_with_cleanup {
    // Case when there is only the condition and the path
    ($cond:expr, $path: expr) => {
        if !$cond {
            test_utils::remove_config_from_path($path);
            panic!("assertion failed: {} is not true", stringify!($cond));
        }
    };
    // Case when there is a condition, custom message, and the path
    ($cond:expr, $msg:expr, $path: expr) => {
        if !$cond {
            test_utils::remove_config_from_path($path);
            panic!(
                "assertion failed: {}",
                $msg
            );
        }
    };
}
