use crate::CvlrLogger;

pub trait CvlrLog {
    fn log(&self, tag: &str, logger: &mut CvlrLogger);
}

#[inline(always)]
pub fn cvlr_log_<T: CvlrLog>(tag: &str, val: &T, logger: &mut CvlrLogger) {
    val.log(tag, logger);
}

#[inline(always)]
pub fn cvlr_log<T: CvlrLog>(tag: &str, val: &T) {
    let mut logger = CvlrLogger::new();
    val.log(tag, &mut logger);
}

#[macro_export]
macro_rules! cvlr_log {
    ($v:expr => $t:expr) => {
        $crate::cvlr_log($t, &($v));
    };

    ($v:expr) => {
        cvlr_log! { $v => stringify!($v) }
    };

    ($v:expr, $( $vs:expr ),+) => {
        cvlr_log! { $v }
        cvlr_log! { $( $vs ),+ }
    };
}

pub use cvlr_log as clog;

impl CvlrLog for u64 {
    fn log(&self, tag: &str, logger: &mut CvlrLogger) {
        logger.log_u64(tag, *self);
    }
}

impl CvlrLog for &u64 {
    fn log(&self, tag: &str, logger: &mut CvlrLogger) {
        (**self).log(tag, logger);
    }
}

impl CvlrLog for &str {
    fn log(&self, _tag: &str, logger: &mut CvlrLogger) {
        logger.log(*self);
    }
}

#[cfg(feature = "mathint")]
impl CvlrLog for mathint::NativeInt {
    fn log(&self, tag: &str, logger: &mut CvlrLogger) {
        logger.log_u64(tag, self.as_internal());
    }
}
