use ansi_colours::ansi256_from_rgb;
use error::{DircolorsError, Result};

#[derive(Debug, Clone, Copy)]
pub enum ColorMode {
    BitDepth24,
    BitDepth8,
}

#[derive(Debug, Clone)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
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

    pub fn get_style(&self, mode: ColorMode) -> String {
        match mode {
            ColorMode::BitDepth24 => format!("2;{r};{g};{b}", r = self.r, g = self.g, b = self.b),
            ColorMode::BitDepth8 => format!(
                "5;{code}",
                code = ansi256_from_rgb((self.r, self.g, self.b))
            ),
        }
    }
}
