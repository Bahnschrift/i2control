use std::{num::ParseIntError, str::FromStr};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseRgbError {
    LengthError,
    ParseIntError(ParseIntError),
}

impl std::fmt::Display for ParseRgbError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseRgbError::LengthError => write!(f, "RGB values must have length 6"),
            ParseRgbError::ParseIntError(parse_int_error) => parse_int_error.fmt(f),
        }
    }
}

impl std::error::Error for ParseRgbError {}

impl From<ParseIntError> for ParseRgbError {
    fn from(parse_int_error: ParseIntError) -> Self {
        Self::ParseIntError(parse_int_error)
    }
}

#[derive(Debug, Clone, Default)]
pub struct Rgb {
    r: u8,
    g: u8,
    b: u8,
}

impl std::fmt::Display for Rgb {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#{:02X}{:02X}{:02X}", self.r, self.g, self.b)
    }
}

impl FromStr for Rgb {
    type Err = ParseRgbError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim_start_matches("#");

        if s.len() == 6 {
            Ok(Self {
                r: u8::from_str_radix(&s[..2], 16)?,
                g: u8::from_str_radix(&s[2..4], 16)?,
                b: u8::from_str_radix(&s[4..], 16)?,
            })
        } else {
            Err(ParseRgbError::LengthError)
        }
    }
}
