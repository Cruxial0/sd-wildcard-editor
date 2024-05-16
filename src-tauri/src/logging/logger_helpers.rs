use super::{
    logger_frontend::{FrontendColor, FrontendConversion},
    logger_settings::{
        DATETIME_COLOR, DEFAULT_COLOR, ERROR_COLOR, SOURCE_COLOR, SOURCE_CUTOFF, SOURCE_INDENT,
    },
};
use colored::{Color, ColoredString, Colorize};

fn adjust_source_length(source: String) -> String {
    let mut src = source;
    if src.len() > SOURCE_INDENT {
        let cropped = src.split_at(SOURCE_INDENT - SOURCE_CUTOFF).0;
        src = format!("{}{}", cropped, "...");
    }

    if src.len() < SOURCE_INDENT {
        let diff = SOURCE_INDENT - src.len();
        let mut whitespace = "".to_owned();
        for _ in 0..diff {
            whitespace.push_str(" ")
        }
        src = format!("{}{}", src, whitespace);
    }

    src
}

/// Creates a tuple with the following structure: (TEXT, color: #HEXCLR;)
fn color_frontend(text: String, color: FrontendColor) -> (String, String) {
    (text, format!("{}{}{}", "color: ", color.as_str(), ";"))
}

pub fn format_source_backend(source: String, is_error: bool) -> ColoredString {
    let src = adjust_source_length(source);

    match is_error {
        true => color_from_enum(src, ERROR_COLOR),
        false => color_from_enum(src, SOURCE_COLOR),
    }
}

pub fn format_datetime_backend(datetime: String, is_error: bool) -> ColoredString {
    match is_error{
        true => color_from_enum(datetime, ERROR_COLOR),
        false => color_from_enum(datetime, DATETIME_COLOR),
    }
}

pub fn format_content_backend(content: String, is_error: bool) -> ColoredString {
    match is_error {
        true => color_from_enum(content, ERROR_COLOR),
        false => color_from_enum(content, DEFAULT_COLOR),
    }
}

pub fn format_source_frontend(source: String, is_error: bool) -> (String, String) {
    let src = adjust_source_length(source);

    match is_error {
        true => color_frontend(src, ERROR_COLOR.to_frontend()),
        false => color_frontend(src, SOURCE_COLOR.to_frontend()),
    }
}

pub fn format_datetime_frontend(datetime: String) -> (String, String) {
    color_frontend(datetime, DATETIME_COLOR.to_frontend())
}

pub fn format_content_frontend(content: String, is_error: bool) -> (String, String) {
    match is_error {
        true => color_frontend(content, ERROR_COLOR.to_frontend()),
        false => color_frontend(content, DEFAULT_COLOR.to_frontend()),
    }
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
