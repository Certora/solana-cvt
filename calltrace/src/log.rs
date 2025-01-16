use crate::CvlrLogger;

pub trait CvlrLog {
    fn log(&self, tag: &str, logger: &mut CvlrLogger);
}


#[macro_export]
macro_rules! cvlr_log {
    ($v: expr) => {{
        let mut logger = $crate::CvlrLogger::new();
        ($v).log(stringify!($v), logger);
    }};
}

pub use cvlr_log as clog;


impl CvlrLog for u64 {
    fn log(&self, tag: &str, logger: &mut CvlrLogger) {
        logger.log_u64(tag, *self);
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
