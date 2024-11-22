#[allow(dead_code)]
pub trait Logger {
    fn log(&mut self, s: &str);
}

#[cfg(feature = "std")]
pub mod std {
    use super::Logger;
    use std::fs::{File, OpenOptions};
    use std::io::prelude::*;
    use std::path::Path;

    pub struct FileLogger {
        outfile: File,
    }

    #[allow(dead_code)]
    impl FileLogger {
        pub fn new(path: &Path, options: &OpenOptions) -> Result<Self, std::io::Error> {
            Ok(Self {
                outfile: options.open(path)?,
            })
        }
    }

    impl Logger for FileLogger {
        fn log(&mut self, s: &str) {
            self.outfile
                .write_all(s.as_bytes())
                .expect("Failed to log to file");
        }
    }

    pub struct TerminalLogger;

    impl TerminalLogger {
        #[allow(dead_code)]
        pub fn new() -> Self {
            Self {}
        }
    }

    impl Logger for TerminalLogger {
        fn log(&mut self, s: &str) {
            println!("{}", s);
        }
    }
}
