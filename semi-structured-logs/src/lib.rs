// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

/// various log levels
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum LogLevel {
    Info,
    Warning,
    Error,
    Debug,
}

/// primary function for emitting logs
pub fn log(level: LogLevel, message: &str) -> String {
    let result = match level {
        LogLevel::Info => info(message),
        LogLevel::Warning => warn(message),
        LogLevel::Error => error(message),
        LogLevel::Debug => debug(message),
    };
    result
}

pub fn info(message: &str) -> String {
    format!("[INFO]: {}", message)
}

pub fn warn(message: &str) -> String {
    format!("[WARNING]: {}", message)
}

pub fn error(message: &str) -> String {
    format!("[ERROR]: {}", message)
}

pub fn debug(message: &str) -> String {
    format!("[DEBUG]: {}", message)
}
