#[derive(serde::Serialize, Clone, PartialEq, PartialOrd)]
pub enum LogLevel {
    FATAL = 0,
    ERROR = 1,
    WARN = 2,
    INFO = 3,
    DEBUG = 4,
    TRACE = 5
}

impl TryFrom<i32> for LogLevel {
    type Error = ();

    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            x if x == LogLevel::FATAL as i32 => Ok(LogLevel::FATAL),
            x if x == LogLevel::ERROR as i32 => Ok(LogLevel::ERROR),
            x if x == LogLevel::WARN as i32 => Ok(LogLevel::WARN),
            x if x == LogLevel::INFO as i32 => Ok(LogLevel::INFO),
            x if x == LogLevel::DEBUG as i32 => Ok(LogLevel::DEBUG),
            x if x == LogLevel::TRACE as i32 => Ok(LogLevel::TRACE),
            _ => Err(()),
        }
    }
}