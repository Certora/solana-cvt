use crate::CvlrLogger;

pub trait CvlrLog {
    fn log(&self, tag: &str, logger: &mut CvlrLogger);
}

pub fn cvlr_log_<T: CvlrLog>(tag: &str, val: &T, logger: &mut CvlrLogger) {
    val.log(tag, logger);
}

#[macro_export]
macro_rules! cvlr_log {
    ($v: expr) => {{
        let mut logger = $crate::CvlrLogger::new();
        $crate::cvlr_log_(stringify!($v), &v, &mut logger);
    }};
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
