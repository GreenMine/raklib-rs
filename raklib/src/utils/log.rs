use colored::Colorize;
use lazy_static::lazy_static;

const PKG_NAME: &str = env!("CARGO_PKG_NAME");
lazy_static! {
    pub static ref DEFAULT_LOGGER: StdoutLogger = {
        let level = std::env::var("LOG_LEVEL")
            .map_or(Ok(Level::Debug), |v| {
                let level_num = v.parse::<i32>();
                if let Err(_) = level_num {
                    return Err(());
                }
                Level::from_num(level_num.unwrap())
            })
            .expect("Error while parsing log level!");

        StdoutLogger { min_level: level }
    };
}

#[derive(Debug, Clone, Copy)]
pub enum Level {
    Debug = 1,
    Info,
    Warning,
    Error,
}

impl Level {
    fn as_str(&self) -> &str {
        match self {
            Level::Info => "info",
            Level::Debug => "debug",
            Level::Warning => "warning",
            Level::Error => "error",
        }
    }

    fn from_num(level: i32) -> Result<Self, ()> {
        return Ok(match level {
            1 => Level::Debug,
            2 => Level::Info,
            3 => Level::Warning,
            4 => Level::Error,
            _ => return Err(()),
        });
    }
}

pub struct StdoutLogger {
    pub min_level: Level,
}

impl StdoutLogger {
    fn log(&self, message: &str, level: Level) {
        if (level as u8) < (self.min_level as u8) {
            return;
        }

        let header = &format!("[{}] [{:?}]", PKG_NAME, level).to_ascii_lowercase()[..];
        let colored_header = match level {
            Level::Debug => header.blue(),
            Level::Info => header.green(),
            Level::Warning => header.yellow(),
            Level::Error => header.red(),
        };

        println!("{} {}", colored_header, message);
    }

    pub fn debug(&self, message: &str) {
        self.log(message, Level::Debug);
    }

    pub fn info(&self, message: &str) {
        self.log(message, Level::Info);
    }

    pub fn warning(&self, message: &str) {
        self.log(message, Level::Warning);
    }

    pub fn error(&self, message: &str) {
        self.log(message, Level::Error);
        std::process::exit(-1);
    }
}

#[macro_export]
macro_rules! debug {
    ($l:ident, $($arg:tt)*) => {
        $l.debug(&format!($($arg)*))
    };
    ($($arg:tt)*) => {
        $crate::log::DEFAULT_LOGGER.debug(&format!($($arg)*))
    };
}

#[macro_export]
macro_rules! log {
    ($l:ident, $($arg:tt)*) => {
        $l.info(&format!($($arg)*))
    };
    ($($arg:tt)*) => {
        $crate::log::DEFAULT_LOGGER.info(&format!($($arg)*))
    };
}

#[macro_export]
macro_rules! warn {
    ($l:ident, $($arg:tt)*) => {
        $l.warning(&format!($($arg)*))
    };
    ($($arg:tt)*) => {
        $crate::log::DEFAULT_LOGGER.warning(&format!($($arg)*))
    };
}

#[macro_export]
macro_rules! error {
    ($l:ident, $($arg:tt)*) => {
        $l.error(&format!($($arg)*))
    };
    ($($arg:tt)*) => {
        $crate::log::DEFAULT_LOGGER.error(&format!($($arg)*))
    };
}
