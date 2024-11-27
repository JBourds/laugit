use core::ptr::addr_of;
use core::sync::atomic::{AtomicBool, Ordering};

static mut LOGGER: LoggerManager = LoggerManager { loggers: &[] };
static mut INITIALIZED: AtomicBool = AtomicBool::new(false);

#[allow(dead_code)]
pub fn logger() -> Option<&'static LoggerManager> {
    unsafe {
        if INITIALIZED.load(Ordering::Relaxed) {
            addr_of!(LOGGER).as_ref()
        } else {
            None
        }
    }
}

#[allow(dead_code)]
pub fn init_logger(loggers: &'static [fn(&str) -> ()]) -> bool {
    unsafe {
        if INITIALIZED.swap(true, Ordering::Relaxed) {
            false
        } else {
            LOGGER = LoggerManager { loggers };
            true
        }
    }
}

#[allow(unused)]
pub struct LoggerManager {
    loggers: &'static [fn(&str) -> ()],
}

#[allow(unused)]
impl LoggerManager {
    pub fn log(&self, s: &str) {
        for logger in self.loggers.iter() {
            logger(s);
        }
    }
}

#[cfg(feature = "std")]
pub mod std {}
