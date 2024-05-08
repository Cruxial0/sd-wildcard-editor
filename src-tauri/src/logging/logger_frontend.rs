use std::borrow::Cow;

use colored::Color;

pub enum FrontendColor{
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite
}

impl FrontendColor {
    pub fn as_str(&self) -> Cow<'static, str> {
        match *self {
            FrontendColor::Black => "#000000".into(),
            FrontendColor::Red => "#00FFFF".into(),
            FrontendColor::Green => "#36A536".into(),
            FrontendColor::Yellow => "#CECE2B".into(),
            FrontendColor::Blue => "#3D3DFF".into(),
            FrontendColor::Magenta => "#C421C4".into(),
            FrontendColor::Cyan => "#23CECE".into(),
            FrontendColor::White => "#FFFFFF".into(),
            FrontendColor::BrightBlack => "#555555".into(),
            FrontendColor::BrightRed => "#FF5858".into(),
            FrontendColor::BrightGreen => "#50F250".into(),
            FrontendColor::BrightYellow => "#F5F539".into(),
            FrontendColor::BrightBlue => "#3D75FF".into(),
            FrontendColor::BrightMagenta => "#F327F3".into(),
            FrontendColor::BrightCyan => "#2BF1F1".into(),
            FrontendColor::BrightWhite => "#FFFFFF".into()
        }
    }
}

pub trait FrontendConversion {
    fn to_frontend(&self) -> FrontendColor;
}

impl FrontendConversion for Color {
    fn to_frontend(&self) -> FrontendColor {
        match *self{
            Color::Black => FrontendColor::Black,
            Color::Red => FrontendColor::Red,
            Color::Green => FrontendColor::Green,
            Color::Yellow => FrontendColor::Yellow,
            Color::Blue => FrontendColor::Blue,
            Color::Magenta => FrontendColor::Magenta,
            Color::Cyan => FrontendColor::Cyan,
            Color::White => FrontendColor::White,
            Color::BrightBlack => FrontendColor::BrightBlack,
            Color::BrightRed => FrontendColor::BrightRed,
            Color::BrightGreen => FrontendColor::BrightGreen,
            Color::BrightYellow => FrontendColor::BrightYellow,
            Color::BrightBlue => FrontendColor::BrightBlue,
            Color::BrightMagenta => FrontendColor::BrightMagenta,
            Color::BrightCyan => FrontendColor::BrightCyan,
            Color::BrightWhite => FrontendColor::BrightWhite,
            Color::TrueColor { .. } => FrontendColor::Black,
        }
    }
}