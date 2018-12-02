use std::collections::HashMap;
use std::path::Path;

use yaml_rust::yaml::YamlLoader;
use yaml_rust::Yaml;

use color::{Color, ColorMode};
use error::{DircolorsError, Result};
use types::Category;
use util::load_yaml_file;

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

pub struct Theme {
    colors: HashMap<String, Color>,
    categories: Yaml, // TODO: load the category tree into a proper data structure
    color_mode: ColorMode,
}

impl Theme {
    pub fn from_file(path: &Path, color_mode: ColorMode) -> Result<Theme> {
        let contents = load_yaml_file(path)
            .map_err(|_| DircolorsError::CouldNotLoadTheme(path.to_string_lossy().into()))?;
        Self::from_string(&contents, color_mode)
    }

    fn from_string(contents: &str, color_mode: ColorMode) -> Result<Theme> {
        let mut docs = YamlLoader::load_from_str(&contents)?;
        let doc = docs.pop().ok_or(DircolorsError::EmptyThemeFile)?;

        let mut colors = HashMap::new();

        match &doc["colors"] {
            Yaml::Hash(map) => {
                for (key, value) in map {
                    match (key, value) {
                        (Yaml::String(key), Yaml::String(value)) => {
                            colors.insert(key.clone(), Color::from_hex_str(&value)?);
                        }
                        _ => return Err(DircolorsError::UnexpectedYamlType),
                    }
                }
            }
            _ => return Err(DircolorsError::UnexpectedYamlType),
        }

        Ok(Theme {
            colors,
            categories: doc,
            color_mode,
        })
    }

    pub fn get_style(&self, category: &Category) -> Result<String> {
        if category.is_empty() {
            // TODO: use a non-empty collection data type to avoid this
            panic!("category should not be empty");
        }

        let mut item = &self.categories;
        for key in category {
            if let Yaml::Hash(map) = item {
                if map.contains_key(&Yaml::String("foreground".into()))
                    || map.contains_key(&Yaml::String("background".into()))
                    || map.contains_key(&Yaml::String("font-style".into()))
                {
                    if map.get(&Yaml::String(key.clone())).is_none() {
                        // We can not specialize further
                        break;
                    }
                }

                if let Some(value) = map.get(&Yaml::String(key.clone())) {
                    item = &value;
                } else {
                    // TODO: warning("could not resolve path '{}'".format("/".join(path)))
                    return Ok("0".into());
                }
            } else {
                return Err(DircolorsError::UnexpectedYamlType);
            }
        }

        if let Yaml::Hash(map) = item {
            let font_style: &str = map
                .get(&Yaml::String("font-style".into()))
                .map(|s| s.as_str().unwrap())
                .unwrap_or("regular");

            let font_style_ansi: &u8 = ANSI_STYLES.get(&font_style).unwrap(); // TODO

            let foreground = map
                .get(&Yaml::String("foreground".into()))
                .map(|s| s.as_str().unwrap())
                .unwrap_or("default");

            let white = Color::white();
            let foreground = self.colors.get(foreground).unwrap_or(&white); // TODO

            let background = map
                .get(&Yaml::String("background".into()))
                .map(|s| s.as_str().unwrap());

            let background = background.and_then(|b| self.colors.get(b));

            let foreground_code = foreground.get_style(self.color_mode);
            let mut style: String = format!(
                "{font_style};38;{foreground_code}",
                font_style = *font_style_ansi,
                foreground_code = foreground_code
            );

            if let Some(background) = background {
                let background_code = background.get_style(self.color_mode);
                style.push_str(&format!(
                    ";48;{background_code}",
                    background_code = background_code
                ));
            }

            Ok(style)
        } else {
            Err(DircolorsError::UnexpectedYamlType)
        }
    }
}
