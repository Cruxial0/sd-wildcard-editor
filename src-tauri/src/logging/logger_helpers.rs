use super::{
    log_level::{self, LogLevel}, logger_colors::{FrontendColor, FrontendConversion, COLORS}, logger_settings::{
        COLOR_DEFAULT, COLOR_ERROR, COLOR_INFO, DATETIME_COLOR, SOURCE_CUTOFF, SOURCE_INDENT,
    }
};
use colored::{Color, ColoredString, Colorize};

pub fn adjust_source_length(source: String, log_level: &LogLevel) -> String {
    let mut src = source;
    let offset = log_level.to_string().len();
    let indent = SOURCE_INDENT - &offset;
    if src.len() > indent {
        let cropped = src.split_at(indent - SOURCE_CUTOFF).0;
        src = format!("{}{}", cropped, "...");
    }

    if src.len() < indent {
        let diff = indent - src.len();
        let mut whitespace = "".to_owned();
        for _ in 0..diff {
            whitespace.push_str(" ")
        }
        src = format!("{}{}", src, whitespace);
    }

    src
}

fn get_color(log_level: &LogLevel) -> Color {
    match COLORS.contains_key(log_level) {
        true => COLORS[log_level],
        false => Color::BrightMagenta,
    }
}

/// Creates a tuple with the following structure: (TEXT, color: #HEXCLR;)
fn _color_frontend(text: String, color: FrontendColor) -> (String, String) {
    (text, format!("{}{}{}", "color: ", color.as_str(), ";"))
}

pub fn format_backend(source: String, log_level: &LogLevel) -> ColoredString {
    color_from_enum(source, get_color(log_level))
}

pub fn color_backend(source: String, color: Color) -> ColoredString{
    color_from_enum(source, color)
}

pub fn format_frontend(source: String, log_level: &LogLevel) -> (String, String) {
    _color_frontend(source, get_color(log_level).to_frontend())
}

pub fn color_frontend(source: String, color: Color) -> (String, String) {
    _color_frontend(source, color.to_frontend())
}

fn color_from_enum(text: String, color: Color) -> ColoredString {
    match color {
        Color::Black => text.black(),
        Color::Red => text.red(),
        Color::Green => text.green(),
        Color::Yellow => text.yellow(),
        Color::Blue => text.blue(),
        Color::Magenta => text.magenta(),
        Color::Cyan => text.cyan(),
        Color::White => text.white(),
        Color::BrightBlack => text.bright_black(),
        Color::BrightRed => text.red(),
        Color::BrightGreen => text.bright_green(),
        Color::BrightYellow => text.bright_yellow(),
        Color::BrightBlue => text.bright_blue(),
        Color::BrightMagenta => text.bright_magenta(),
        Color::BrightCyan => text.bright_cyan(),
        Color::BrightWhite => text.bright_white(),
        Color::TrueColor { r, g, b } => text.truecolor(r, g, b),
    }
}
