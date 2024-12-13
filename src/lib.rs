#![cfg_attr(not(feature = "std"), no_std)]
use thiserror_no_std::Error;

use core::cmp;
use core::ptr::addr_of;
use core::sync::atomic::{AtomicBool, Ordering};

static mut LOGGER: LoggerManager = LoggerManager {
    loggers: &[],
    level: LogLevel::Silent,
};
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
pub fn init_logger(level: LogLevel, loggers: &'static [fn(&str) -> ()]) -> Result<(), LogError> {
    unsafe {
        if INITIALIZED.swap(true, Ordering::Relaxed) {
            Err(LogError::LoggerAlreadyInitialized)
        } else {
            LOGGER = LoggerManager { loggers, level };
            Ok(())
        }
    }
}

#[allow(unused)]
#[derive(Debug)]
pub struct LoggerManager {
    loggers: &'static [fn(&str) -> ()],
    level: LogLevel,
}

#[allow(unused)]
impl LoggerManager {
    pub fn log(&self, s: &str) {
        for &logger in self.loggers.iter() {
            logger(s);
        }
    }
}

#[repr(u8)]
#[derive(Debug, Eq)]
pub enum LogLevel {
    Debug,
    Info,
    Notice,
    Warning,
    Error,
    Critical,
    Alert,
    Emergency,
    Silent,
}

impl LogLevel {
    const NUM_LOG_LEVELS: usize = Self::MESSAGES.len();
    const MESSAGES: &'static [&'static str] = &[
        "DEBUG",
        "INFO",
        "NOTICE",
        "WARNING",
        "ERROR",
        "CRITICAL",
        "ALERT",
        "EMERGENCY",
        "SILENT",
    ];
}

impl Ord for LogLevel {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        usize::from(self).cmp(&usize::from(other))
    }
}

impl PartialOrd for LogLevel {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for LogLevel {
    fn eq(&self, other: &Self) -> bool {
        usize::from(self) == usize::from(other)
    }
}

impl TryFrom<usize> for LogLevel {
    type Error = ();
    fn try_from(value: usize) -> Result<Self, <Self as TryFrom<usize>>::Error> {
        match value {
            0 => Ok(LogLevel::Debug),
            1 => Ok(LogLevel::Info),
            2 => Ok(LogLevel::Notice),
            3 => Ok(LogLevel::Warning),
            4 => Ok(LogLevel::Error),
            5 => Ok(LogLevel::Critical),
            6 => Ok(LogLevel::Alert),
            7 => Ok(LogLevel::Emergency),
            8 => Ok(LogLevel::Silent),
            _ => Err(()),
        }
    }
}

impl From<&LogLevel> for usize {
    fn from(level: &LogLevel) -> usize {
        match *level {
            LogLevel::Debug => 0,
            LogLevel::Info => 1,
            LogLevel::Notice => 2,
            LogLevel::Warning => 3,
            LogLevel::Error => 4,
            LogLevel::Critical => 5,
            LogLevel::Alert => 6,
            LogLevel::Emergency => 7,
            LogLevel::Silent => 8,
        }
    }
}

impl TryFrom<&LogLevel> for &'static str {
    type Error = ();
    fn try_from(level: &LogLevel) -> Result<Self, Self::Error> {
        let index = usize::from(level);
        if index < LogLevel::MESSAGES.len() {
            Ok(LogLevel::MESSAGES[usize::from(level)])
        } else {
            Err(())
        }
    }
}

impl TryFrom<&str> for LogLevel {
    type Error = ();
    fn try_from(s: &str) -> Result<Self, <LogLevel as TryFrom<&str>>::Error> {
        for i in 0..Self::NUM_LOG_LEVELS {
            if Self::MESSAGES[i] == s {
                return Ok(Self::try_from(i)
                    .expect("Mismatch between LogLevel message array and number of variants."));
            }
        }
        Err(())
    }
}

#[derive(Debug, Error, PartialEq)]
pub enum LogError {
    #[error("Logger already initialized.")]
    LoggerAlreadyInitialized,
}

#[cfg(test)]
mod no_std_tests {
    use super::{LogLevel, LoggerManager};
    #[test]
    fn test_log() {
        static mut BUFFER_1: [u8; 64] = [0; 64];
        static mut BUFFER_2: [u8; 64] = [0; 64];
        fn log_fn1(s: &str) {
            for (i, c) in "Logger 1:"
                .as_bytes()
                .iter()
                .chain(s.as_bytes().iter())
                .enumerate()
            {
                unsafe {
                    BUFFER_1[i] = *c;
                }
            }
        }
        fn log_fn2(s: &str) {
            for (i, c) in "Logger 2:"
                .as_bytes()
                .iter()
                .chain(s.as_bytes().iter())
                .enumerate()
            {
                unsafe {
                    BUFFER_2[i] = *c;
                }
            }
        }
        let loggers = &[log_fn1, log_fn2];
        let logger = LoggerManager {
            level: LogLevel::Debug,
            loggers,
        };
        logger.log("test");
        unsafe {
            let buffer_1_expected = "Logger 1:test".as_bytes();
            let buffer_2_expected = "Logger 2:test".as_bytes();
            for i in 0..buffer_1_expected.len() {
                assert_eq!(buffer_1_expected[i], BUFFER_1[i]);
                assert_eq!(buffer_2_expected[i], BUFFER_2[i]);
            }
            for i in buffer_1_expected.len()..BUFFER_1.len() {
                assert_eq!(0, BUFFER_1[i]);
                assert_eq!(0, BUFFER_2[i]);
            }
        }
    }
}

#[cfg(all(feature = "std", test))]
mod std_tests {
    use super::{LogLevel, LoggerManager};
    #[test]
    fn test_print() {
        fn log_fn1(s: &str) {
            println!("Log Function 1: {}", s);
        }
        fn log_fn2(s: &str) {
            println!("Log Function 2: {}", s);
        }
        let loggers = &[log_fn1, log_fn2];
        let logger = LoggerManager {
            level: LogLevel::Debug,
            loggers,
        };
        logger.log("test");
    }
}
