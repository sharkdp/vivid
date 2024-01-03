use std::collections::HashMap;
use std::path::Path;

use yaml_rust::yaml::YamlLoader;
use yaml_rust::Yaml;

use crate::color::{Color, ColorMode, ColorType};
use crate::error::{Result, VividError};
use crate::font_style::FontStyle;
use crate::types::CategoryRef;
use crate::util::{load_yaml_file, transpose};

#[derive(Debug)]
pub struct Theme {
    colors: HashMap<String, Color>,
    categories: Yaml, // TODO: load the category tree into a proper data structure
    color_mode: ColorMode,
}

impl Theme {
    pub fn from_path(path: &Path, color_mode: ColorMode) -> Result<Theme> {
        let contents = load_yaml_file(path)
            .map_err(|_| VividError::CouldNotLoadTheme(path.to_string_lossy().into()))?;
        Self::from_string(&contents, color_mode)
    }

    pub(crate) fn from_string(contents: &str, color_mode: ColorMode) -> Result<Theme> {
        let mut docs = YamlLoader::load_from_str(contents)?;
        let doc = docs.pop().ok_or(VividError::EmptyThemeFile)?;

        let mut colors = HashMap::new();

        match &doc["colors"] {
            Yaml::Hash(map) => {
                for (key, value) in map {
                    match (key, value) {
                        (Yaml::String(key), Yaml::String(value)) => {
                            colors.insert(key.clone(), value.parse()?);
                        }
                        _ => return Err(VividError::UnexpectedYamlType),
                    }
                }
            }
            _ => return Err(VividError::UnexpectedYamlType),
        }

        Ok(Theme {
            colors,
            categories: doc,
            color_mode,
        })
    }

    fn get_color(&self, color_str: &str) -> Result<Color> {
        self.colors
            .get(color_str)
            .cloned()
            .or_else(|| color_str.parse().ok())
            .ok_or_else(|| VividError::UnknownColor(color_str.to_string()))
    }

    pub fn get_style(&self, category: CategoryRef) -> Result<String> {
        if category.is_empty() {
            // TODO: use a non-empty collection data type to avoid this
            panic!("category should not be empty");
        }

        let mut item = &self.categories;
        for key in category {
            if let Yaml::Hash(map) = item {
                if (map.contains_key(&Yaml::String("foreground".into()))
                    || map.contains_key(&Yaml::String("background".into()))
                    || map.contains_key(&Yaml::String("font-style".into())))
                    && map.get(&Yaml::String(key.clone())).is_none()
                {
                    // We can not specialize further
                    break;
                }

                if let Some(value) = map.get(&Yaml::String(key.clone())) {
                    item = value;
                } else {
                    return Err(VividError::CouldNotFindStyleFor(category.join(".")));
                }
            } else {
                return Err(VividError::UnexpectedYamlType);
            }
        }

        if let Yaml::Hash(map) = item {
            let font_style = FontStyle::from_yaml(map);

            let foreground = map
                .get(&Yaml::String("foreground".into()))
                .map(|s| s.as_str().unwrap());

            let foreground = transpose(foreground.map(|fg| self.get_color(fg)))?;

            let background = map
                .get(&Yaml::String("background".into()))
                .map(|s| s.as_str().expect("'background' value should be a string"));

            let background = transpose(background.map(|fg| self.get_color(fg)))?;

            let mut style: String = format!("{font_style}");
            if let Some(foreground) = foreground {
                let foreground_code = foreground.get_style(ColorType::Foreground, self.color_mode);
                style.push_str(&format!(
                    ";{foreground_code}",
                    foreground_code = foreground_code
                ));
            }

            if let Some(background) = background {
                let background_code = background.get_style(ColorType::Background, self.color_mode);
                style.push_str(&format!(
                    ";{background_code}",
                    background_code = background_code
                ));
            }

            Ok(style)
        } else {
            Err(VividError::UnexpectedYamlType)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Theme;
    use crate::color::ColorMode;

    #[test]
    fn basic() {
        let theme = Theme::from_string(
            "
                colors:
                  color1: '00ff7f'

                foo:
                  bar:
                    foreground: color1

                c1:
                  foreground: 'ffffff'

                  c2:
                    foreground: '000000'

                t3:
                    font-style: bold",
            ColorMode::BitDepth24,
        )
        .unwrap();

        let style1 = theme.get_style(&["foo".into(), "bar".into()]).unwrap();
        assert_eq!("0;38;2;0;255;127", style1);

        let style2 = theme
            .get_style(&["c1".into(), "c2".into(), "c3".into()])
            .unwrap();
        assert_eq!("0;38;2;0;0;0", style2);

        let style3 = theme.get_style(&["t3".into()]).unwrap();
        assert_eq!("1", style3);
    }
}
