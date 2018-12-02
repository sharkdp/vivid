use ansi_colours::ansi256_from_rgb;
use error::{DircolorsError, Result};

#[derive(Debug, Clone, Copy)]
pub enum ColorMode {
    BitDepth24,
    BitDepth8,
}

#[derive(Debug, Clone, Copy)]
pub enum ColorType {
    Foreground,
    Background,
}

impl ColorType {
    fn get_code(self) -> &'static str {
        match self {
            ColorType::Foreground => "38",
            ColorType::Background => "48",
        }
    }
}

#[derive(Debug, Clone)]
pub enum Color {
    Default,
    RGB(u8, u8, u8),
}

impl Color {
    pub fn from_hex_str(hex_str: &str) -> Result<Color> {
        let parse_error = || DircolorsError::ColorParseError(hex_str.to_string());

        if hex_str.len() != 6 {
            return Err(parse_error());
        }

        let r = u8::from_str_radix(&hex_str[0..2], 16).map_err(|_| parse_error())?;
        let g = u8::from_str_radix(&hex_str[2..4], 16).map_err(|_| parse_error())?;
        let b = u8::from_str_radix(&hex_str[4..6], 16).map_err(|_| parse_error())?;

        Ok(Color::RGB(r, g, b))
    }

    pub fn get_style(&self, colortype: ColorType, colormode: ColorMode) -> String {
        match self {
            Color::Default => String::default(),
            Color::RGB(r, g, b) => match colormode {
                ColorMode::BitDepth24 => format!(
                    "{ctype};2;{r};{g};{b}",
                    ctype = colortype.get_code(),
                    r = r,
                    g = g,
                    b = b
                ),
                ColorMode::BitDepth8 => format!(
                    "{ctype};5;{code}",
                    ctype = colortype.get_code(),
                    code = ansi256_from_rgb((*r, *g, *b))
                ),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Color, ColorMode, ColorType};

    #[test]
    fn default() {
        let style = Color::Default.get_style(ColorType::Foreground, ColorMode::BitDepth24);
        assert_eq!("", style);
    }

    #[test]
    fn fg_white() {
        let white = Color::RGB(0xff, 0xff, 0xff);
        let style_8bit = white.get_style(ColorType::Foreground, ColorMode::BitDepth8);
        assert_eq!("38;5;231", style_8bit);

        let style_24bit = white.get_style(ColorType::Foreground, ColorMode::BitDepth24);
        assert_eq!("38;2;255;255;255", style_24bit);
    }

    #[test]
    fn bg_black() {
        let black = Color::RGB(0x00, 0x00, 0x00);
        let style_8bit = black.get_style(ColorType::Background, ColorMode::BitDepth8);
        assert_eq!("48;5;16", style_8bit);

        let style_24bit = black.get_style(ColorType::Background, ColorMode::BitDepth24);
        assert_eq!("48;2;0;0;0", style_24bit);
    }

    #[test]
    fn fg_red() {
        let red = Color::RGB(0xff, 0x00, 0x00);
        let style_8bit = red.get_style(ColorType::Foreground, ColorMode::BitDepth8);
        assert_eq!("38;5;196", style_8bit);

        let style_24bit = red.get_style(ColorType::Foreground, ColorMode::BitDepth24);
        assert_eq!("38;2;255;0;0", style_24bit);
    }
}
