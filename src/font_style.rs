use std::{
    collections::HashMap,
    fmt::{self, Display, Formatter},
};

use lazy_static::lazy_static;

use yaml_rust::{yaml::Hash, Yaml};

use crate::error::{Result, VividError};

lazy_static! {
    static ref ANSI_STYLES: HashMap<&'static str, u8> = {
        let mut m = HashMap::new();
        m.insert("regular", 0);
        m.insert("bold", 1);
        m.insert("faint", 2);
        m.insert("italic", 3);
        m.insert("underline", 4);
        m.insert("blink", 5);
        m.insert("rapid-blink", 6);
        m.insert("overline", 53);
        m
    };
}

/// A list of font styles
#[derive(Default)]
pub struct FontStyle(Vec<u8>);

impl FontStyle {
    /// Creates a FontStyle from the yaml
    ///
    /// # Errors
    ///
    /// Returns an error if the `font-style` value is neither a string nor an
    /// array of strings, or if a style name is not a known font style.
    pub fn from_yaml(map: &Hash) -> Result<Self> {
        match map.get(&Yaml::String("font-style".into())) {
            Some(value) => match value {
                Yaml::String(_) => Ok(Self(vec![Self::style_code(value)?])),
                Yaml::Array(array) => array
                    .iter()
                    .map(Self::style_code)
                    .collect::<Result<Vec<_>>>()
                    .map(Self),
                _ => Err(VividError::UnexpectedYamlTypeFor("font-style")),
            },
            None => Ok(Self(vec![0])),
        }
    }

    /// Returns the ANSI code for a single font style yaml value
    fn style_code(item: &Yaml) -> Result<u8> {
        let name = item
            .as_str()
            .ok_or(VividError::UnexpectedYamlTypeFor("font-style"))?;
        ANSI_STYLES
            .get(name)
            .copied()
            .ok_or_else(|| VividError::UnknownFontStyle(name.to_string()))
    }
}

impl Display for FontStyle {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for (i, style) in self.0.iter().enumerate() {
            if i + 1 == self.0.len() {
                write!(f, "{}", style)?;
            } else {
                write!(f, "{};", style)?;
            }
        }
        Ok(())
    }
}
