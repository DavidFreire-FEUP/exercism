// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

/// various log levels
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum LogLevel {
    Info,
    Warning,
    Error,
    Debug
}
/// primary function for emitting logs
pub fn log(level: LogLevel, message: &str) -> String {
    let log_prefix: String;
    match level {
        LogLevel::Info => log_prefix = "[INFO]: ".to_string(),
        LogLevel::Warning => log_prefix = "[WARNING]: ".to_string(),
        LogLevel::Error => log_prefix = "[ERROR]: ".to_string(),
        LogLevel::Debug => log_prefix = "[DEBUG]: ".to_string(),
    }
    return log_prefix + message;
}
pub fn info(message: &str) -> String {
    return log(LogLevel::Info, message);
}
pub fn warn(message: &str) -> String {
    return log(LogLevel::Warning, message);
}
pub fn error(message: &str) -> String {
    return log(LogLevel::Error, message);
}
