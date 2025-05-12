use std::num::ParseIntError;

#[macro_export]
macro_rules! get_dump {
    ($path:literal) => {
        crate::dump::hex_from_dump(include_str!($path))?.as_slice()
    };
}

pub fn hex_from_dump(dump: &str) -> Result<Vec<u8>, ParseIntError> {
    let mut dat = Vec::new();
    for line in dump.lines() {
        for val in line.split_ascii_whitespace().skip(1) {
            dat.push(u8::from_str_radix(val, 16)?);
        }
    }

    Ok(dat)
}
