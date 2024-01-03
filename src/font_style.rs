use std::{
    collections::HashMap,
    fmt::{self, Display, Formatter},
};

use lazy_static::lazy_static;

use yaml_rust::{yaml::Hash, Yaml};

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
    /// # Panics
    ///
    /// Panics if the yaml value is neither a string or
    /// a yaml array
    pub fn from_yaml(map: &Hash) -> Self {
        match map.get(&Yaml::String("font-style".into())) {
            Some(value) => match value {
                Yaml::String(s) => Self(vec![ANSI_STYLES[s.as_str()]]),
                Yaml::Array(array) => {
                    let mut vec = Vec::with_capacity(array.len());
                    for item in array {
                        vec.push(
                            ANSI_STYLES[item
                                .as_str()
                                .expect("font_style should be a string or an array of strings")],
                        );
                    }
                    Self(vec)
                }
                _ => panic!("font-style should be a string or an array of strings"),
            },
            None => Self(vec![0]),
        }
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
