#![cfg_attr(not(feature = "std"), no_std)]

use core::cmp::Ordering;
mod loggers;

#[macro_export]
macro_rules! log {
    (
        $logger:expr,
        $event:literal,
        $level:expr,
        $($arg:tt)*
    ) => {
        let message = format!(
            "{} {}: {}",
            $event,
            <&'static str>::try_from(&$level)
                .expect("Missing function to convert log level to string."),
            format_args!($($arg)*),
        );
        let message = message.as_str();
        $logger.log(message);
    };
}

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

// TODO: Better errors
mod errors {}

#[cfg(test)]
mod tests {
    use super::LogLevel;

    #[test]
    fn test_log_level_cmp() {
        for cmp in 0..8 {
            let level1 = LogLevel::try_from(cmp).unwrap();
            for other in 0..8 {
                let level2 = LogLevel::try_from(other).unwrap();
                assert_eq!(level1 == level2, cmp == other);
                assert_eq!(level1 < level2, cmp < other);
                assert_eq!(level1 > level2, cmp > other);
                assert_eq!(level1 <= level2, cmp <= other);
                assert_eq!(level1 >= level2, cmp >= other);
            }
        }

        let expected = Err(());
        assert_eq!(expected, LogLevel::try_from(9));
    }
}

#[cfg(all(feature = "std", test))]
mod std_tests {
    use super::{
        loggers::{std::*, Logger},
        LogLevel,
    };
    use std::fs::{remove_file, OpenOptions};
    use std::path::Path;

    #[test]
    fn terminal_logger() {
        let mut logger = TerminalLogger::new();
        log!(logger, "test_terminal_logger", LogLevel::Debug, "Testing!");
    }

    #[test]
    fn file_logger() {
        let path = Path::new("test.log");
        let mut options = OpenOptions::new();
        options.read(true).write(true).create(true).truncate(true);
        let mut logger = FileLogger::new(path, &options).unwrap();
        log!(logger, "test_file_logger", LogLevel::Debug, "Testing!");
        remove_file(path).unwrap();
    }

    #[test]
    fn ignore_unused_events() {}
}
