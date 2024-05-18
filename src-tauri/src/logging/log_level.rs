#[derive(serde::Serialize, Clone, PartialEq, PartialOrd, Eq, Hash)]
#[repr(i32)]
pub enum LogLevel {
    Custom(String) = -1,
    FATAL = 0,
    ERROR = 1,
    WARN = 2,
    INFO = 3,
    DEBUG = 4,
    TRACE = 5,
}

impl LogLevel {
    pub fn to_string(&self) -> String {
        match self {
            LogLevel::FATAL => "FATAL".to_owned(),
            LogLevel::ERROR => "ERROR".to_owned(),
            LogLevel::WARN => "WARN".to_owned(),
            LogLevel::INFO => "INFO".to_owned(),
            LogLevel::DEBUG => "DEBUG".to_owned(),
            LogLevel::TRACE => "TRACE".to_owned(),
            LogLevel::Custom(x) => x.to_uppercase().to_owned(),
        }
    }
}