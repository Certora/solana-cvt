
use cvt_core::CVT_assume;

// We redefine this macro to avoid including error code conversion/formatting
#[macro_export]
macro_rules! require_keys_eq {
    ($value1: expr, $value2: expr, $error_code:expr $(,)?) => {
        CVT_assume($value1 == $value2);
    };
    ($value1: expr, $value2: expr $(,)?) => {
        CVT_assume($value1 == $value2);
    };
}

#[macro_export]
macro_rules! require_keys_neq {
    ($value1: expr, $value2: expr, $error_code:expr $(,)?) => {
        CVT_assume($value1 != $value2);
    };
    ($value1: expr, $value2: expr $(,)?) => {
        CVT_assume($value1 != $value2);
    };
}