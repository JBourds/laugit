use core::cmp::Ordering;
use std::env;

fn main() {
    // Comma-separated list of event names to trace
    if let Ok(events) = env::var("LOGOS_EVENTS") {
        for event in events.split(',').map(str::trim) {
            println!("cargo::rustc-cfg=key=\"{}\"", event);
            println!("cargo::warning=\"Logging Events: {}\"", event);
        }
    }
    // Integer value from 0 to max log level - if nothing is passed in then
    // just don't include any logs (> max log level)
    if let Ok(level_str) = env::var("LOGOS_LEVEL") {
        let log_level = LogLevel::try_from(level_str.as_str());
        let level = {
            if let Ok(level) = log_level {
                usize::from(&level)
            } else {
                LogLevel::NUM_LOG_LEVELS
            }
        };

        for l in level..LogLevel::NUM_LOG_LEVELS {
            if let Ok(log_level) = LogLevel::try_from(l) {
                if let Ok(s) = <&'static str>::try_from(&log_level) {
                    println!("cargo::rustc-cfg=key=\"{}\"", s);
                    println!("cargo::warning=\"Enabled Log Level: {}\"", s);
                }
            }
        }
    }
}

// Copy & pasted from library to use here
#[derive(Debug, Eq)]
pub enum LogLevel {
    Debug,
    Informational,
    Notice,
    Warning,
    Error,
    Critical,
    Alert,
    Emergency,
}

impl LogLevel {
    const MESSAGES: &'static [&'static str] = &[
        "Debug",
        "Informational",
        "Notice",
        "Warning",
        "Error",
        "Critical",
        "Alert",
        "Emergency",
    ];
    const NUM_LOG_LEVELS: usize = Self::MESSAGES.len();
}

impl Ord for LogLevel {
    fn cmp(&self, other: &Self) -> Ordering {
        usize::from(self).cmp(&usize::from(other))
    }
}

impl PartialOrd for LogLevel {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
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
            1 => Ok(LogLevel::Informational),
            2 => Ok(LogLevel::Notice),
            3 => Ok(LogLevel::Warning),
            4 => Ok(LogLevel::Error),
            5 => Ok(LogLevel::Critical),
            6 => Ok(LogLevel::Alert),
            7 => Ok(LogLevel::Emergency),
            _ => Err(()),
        }
    }
}

impl From<&LogLevel> for usize {
    fn from(level: &LogLevel) -> usize {
        match *level {
            LogLevel::Debug => 0,
            LogLevel::Informational => 1,
            LogLevel::Notice => 2,
            LogLevel::Warning => 3,
            LogLevel::Error => 4,
            LogLevel::Critical => 5,
            LogLevel::Alert => 6,
            LogLevel::Emergency => 7,
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
