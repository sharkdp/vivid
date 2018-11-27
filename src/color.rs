use error::{DircolorsError, Result};

#[derive(Debug, Clone)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn white() -> Color {
        Color {
            r: 0xff,
            g: 0xff,
            b: 0xff,
        }
    }

    pub fn from_hex_str(hex_str: &str) -> Result<Color> {
        if hex_str.len() != 6 {
            return Err(DircolorsError::ColorParseError);
        }

        let r = u8::from_str_radix(&hex_str[0..2], 16)?;
        let g = u8::from_str_radix(&hex_str[2..4], 16)?;
        let b = u8::from_str_radix(&hex_str[4..6], 16)?;

        Ok(Color { r, g, b })
    }
}
