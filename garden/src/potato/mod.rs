use std::fmt::{Display, Formatter};
use std::process::exit;
use colored::Colorize;

use crate::mem::MutRef;

////////////////////////////////////////////////////////////////////////////////////////////////////
//                                             Macros                                             //
////////////////////////////////////////////////////////////////////////////////////////////////////

#[macro_export]
macro_rules! critical {
    ($label:expr, $($message:tt)*) => {
        $crate::logger::Logger::get().critical($label, format!($($message)*).as_str())
    };
}

#[macro_export]
macro_rules! error {
    ($label:expr, $($message:tt)*) => {
        $crate::logger::Logger::get().error($label, format!($($message)*).as_str())
    };
}

#[macro_export]
macro_rules! warn {
    ($label:expr, $($message:tt)*) => {
        $crate::logger::Logger::get().warn($label, format!($($message)*).as_str())
    };
}

#[macro_export]
macro_rules! info {
    ($label:expr, $($message:tt)*) => {
        $crate::logger::Logger::get().info($label, format!($($message)*).as_str())
    };
}


#[macro_export]
macro_rules! debug {
    ($label:expr, $($message:tt)*) => {
        $crate::logger::Logger::get().debug($label, format!($($message)*).as_str())
    };
}

////////////////////////////////////////////////////////////////////////////////////////////////////
//                                            Log Level                                           //
////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Ord, PartialOrd, Eq, PartialEq)]
pub enum LogLevel {
    Critical,
    Error,
    Warning,
    Info,
    Debug
}

impl Display for LogLevel {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            LogLevel::Critical => write!(f, "CRITICAL"),
            LogLevel::Error => write!(f, "ERROR"),
            LogLevel::Warning => write!(f, "WARNING"),
            LogLevel::Info => write!(f, "INFO"),
            LogLevel::Debug => write!(f, "DEBUG"),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
//                                               Log                                              //
////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct Log {
    level: LogLevel,
    label: String,
    message: String
}

impl Log {

    pub fn new(level: LogLevel, label: String, message: String) -> Log {
        return Log {
            level,
            label,
            message
        }
    }

}

impl Display for Log {

    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] {} - {}", self.level, self.label, self.message)?;
        Ok(())
    }

}

////////////////////////////////////////////////////////////////////////////////////////////////////
//                                             Logger                                             //
////////////////////////////////////////////////////////////////////////////////////////////////////

static mut LOGGER: Option<Logger> = None;

pub struct Logger {
    pub level: LogLevel
}

impl Logger {

    pub fn get() -> MutRef<Logger> {
        unsafe {
            return if let Some(logger) = &mut LOGGER {
                MutRef::from(logger)
            } else {
                colored::control::set_override(true);
                LOGGER = Some(Logger::new());
                MutRef::from(LOGGER.as_mut().unwrap())
            }
        }
    }

    fn new() -> Logger {
        return Logger {
            level: LogLevel::Info
        }
    }

    pub fn log(&mut self, log: Log) {
        if log.level > self.level { return; }

        match log.level {
            LogLevel::Critical => {
                println!("{}", log.to_string().as_str().purple());
                exit(-1);
            },
            LogLevel::Error => println!("{}", log.to_string().as_str().red()),
            LogLevel::Warning => println!("{}", log.to_string().as_str().yellow()),
            LogLevel::Info => println!("{}", log.to_string().as_str().blue()),
            LogLevel::Debug => println!("{}", log.to_string().as_str().truecolor(125, 125, 125)),
        }
    }

    pub fn critical<T: ToString>(&mut self, label: T, message: T) -> ! {
        self.log(Log::new(LogLevel::Critical, label.to_string(), message.to_string()));
        exit(-1);
    }

    pub fn error<T: ToString>(&mut self, label: T, message: T) {
        self.log(Log::new(LogLevel::Error, label.to_string(), message.to_string()));
    }

    pub fn warn<T: ToString>(&mut self, label: T, message: T) {
        self.log(Log::new(LogLevel::Warning, label.to_string(), message.to_string()));
    }

    pub fn info<T: ToString>(&mut self, label: T, message: T) {
        self.log(Log::new(LogLevel::Info, label.to_string(), message.to_string()));
    }

    pub fn debug<T: ToString>(&mut self, label: T, message: T) {
        self.log(Log::new(LogLevel::Debug, label.to_string(), message.to_string()));
    }

    pub fn set_level(&mut self, level: LogLevel) {
        self.level = level;
    }

}

////////////////////////////////////////////////////////////////////////////////////////////////////
//                                          Log Expect                                            //
////////////////////////////////////////////////////////////////////////////////////////////////////

pub trait Assume<T> {

    fn assume(self, label: &str, msg: &str) -> T;

}

impl<T> Assume<T> for Option<T> {

    fn assume(self, label: &str, msg: &str) -> T {
        match self {
            None => critical!(label, "{}", msg),
            Some(value) => value
        }
    }

}

impl<T, E: Display> Assume<T> for Result<T, E> {

    fn assume(self, label: &str, msg: &str) -> T {
        match self {
            Ok(value) => value,
            Err(error) => critical!(label, "{}, {}", msg, error),
        }
    }

}