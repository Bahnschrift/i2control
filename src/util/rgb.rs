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

/// RGB value, represented as three u8s.
/// Example:
/// ```
/// let rgb: Rgb = "#FF0005".parse().unwrap();
/// println!("{rgb}, {rgb:?}"); // #FF0005, Rgb { r: 255, g: 0, b: 5 }
/// ```
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Rgb {
    r: u8,
    g: u8,
    b: u8,
}

impl Rgb {
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Rgb { r, g, b }
    }

    pub const fn bytes(&self) -> [u8; 3] {
        [self.r, self.g, self.b]
    }
}

/// Creates a new RGB value from the given values.
/// Example:
/// ```
/// let col1 = rgb!(0xFF, 0x00, 0x50);
/// let col2 = rgb!(0xDEADBE);
/// println!("{col1}, {col2}"); // #FF0050, #DEADBE
/// ```
#[macro_export]
macro_rules! rgb {
    ($r:expr, $g:expr, $b:expr) => {
        Rgb::new($r, $g, $b)
    };

    ($rgb:expr) => {{
        let rgb = $rgb as u64;
        let r = ((rgb & 0xFF0000) >> 16) as u8;
        let g = ((rgb & 0x00FF00) >> 8) as u8;
        let b = (rgb & 0x0000FF) as u8;
        Rgb::new(r, g, b)
    }};
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
