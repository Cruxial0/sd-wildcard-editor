use colored::Color;

//[DATETIME] {SOURCE} | {CONTENT}
pub const LOG_FORMAT: &str = "[{}] {} | {}";

pub const DATETIME_FORMAT: &str = "%H:%M:%S";
pub const DATETIME_INDENT: usize = 16;
pub const DATETIME_COLOR: Color = Color::BrightBlack;

pub const SOURCE_INDENT: usize = 16;
pub const SOURCE_CUTOFF: usize = 3;
pub const SOURCE_COLOR: Color = Color::Cyan;

pub const ERROR_COLOR: Color = Color::BrightRed;
pub const DEFAULT_COLOR: Color = Color::White;