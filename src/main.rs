extern crate yaml_rust;
#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;
use std::error::Error;
use std::fmt::Display;
use std::fs::File;
use std::io::{self, Read};
use std::num::ParseIntError;
use std::path::Path;
use std::result;

use yaml_rust::yaml::YamlLoader;
use yaml_rust::{ScanError, Yaml};

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
        m.insert("overline", 5);
        m
    };
}

#[derive(Debug)]
enum DircolorsError {
    IoError(io::Error),
    YamlParsingError(ScanError),
    UnexpectedYamlType,
    ColorParseError,
}

impl Display for DircolorsError {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> result::Result<(), std::fmt::Error> {
        write!(fmt, "{}", self.description())
    }
}

impl Error for DircolorsError {
    fn description(&self) -> &str {
        match self {
            DircolorsError::IoError(e) => e.description(),
            DircolorsError::YamlParsingError(e) => e.description(),
            DircolorsError::UnexpectedYamlType => "Unexpected type in YAML file",
            DircolorsError::ColorParseError => "Could not parse color string",
        }
    }
}

impl From<io::Error> for DircolorsError {
    fn from(e: io::Error) -> Self {
        DircolorsError::IoError(e)
    }
}

impl From<ScanError> for DircolorsError {
    fn from(e: ScanError) -> Self {
        DircolorsError::YamlParsingError(e)
    }
}

impl From<ParseIntError> for DircolorsError {
    fn from(e: ParseIntError) -> Self {
        DircolorsError::ColorParseError
    }
}

type Result<T> = std::result::Result<T, DircolorsError>;

type FileType = String;
type Category = Vec<String>;

struct FileTypesMapping {
    mapping: HashMap<FileType, Category>,
}

fn load_yaml_file(path: &Path) -> Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

impl FileTypesMapping {
    fn from_file(path: &Path) -> Result<FileTypesMapping> {
        let contents = load_yaml_file(path)?;
        Self::from_string(&contents)
    }

    fn from_string(contents: &str) -> Result<FileTypesMapping> {
        let docs = YamlLoader::load_from_str(&contents)?;
        let doc = &docs[0];

        Self::get_mapping(doc, vec![])
    }

    fn get_code(filetype: &str) -> String {
        if filetype.get(0..1) == Some("$") {
            filetype[1..].into()
        } else {
            let mut s = String::from("*");
            s.push_str(filetype);
            s
        }
    }

    fn get_mapping(value: &Yaml, category: Category) -> Result<FileTypesMapping> {
        let mut mapping = HashMap::new();

        match value {
            Yaml::Array(array) => {
                for filetype in array {
                    if let Yaml::String(filetype) = filetype {
                        let code = Self::get_code(filetype);
                        mapping.insert(code, category.clone());
                    } else {
                        return Err(DircolorsError::UnexpectedYamlType);
                    }
                }
            }
            Yaml::Hash(ref map) => {
                for (key, value) in map {
                    let mut child_category = category.clone();
                    if let Yaml::String(key) = key {
                        child_category.push(key.clone());
                    }
                    let child_mapping = Self::get_mapping(value, child_category)?;
                    mapping.extend(child_mapping.mapping);
                }
            }
            _ => {
                return Err(DircolorsError::UnexpectedYamlType);
            }
        }

        Ok(FileTypesMapping { mapping })
    }
}

#[derive(Debug, Clone)]
struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    fn white() -> Color {
        Color {
            r: 0xff,
            g: 0xff,
            b: 0xff,
        }
    }

    fn from_hex_str(hex_str: &str) -> Result<Color> {
        if hex_str.len() != 6 {
            return Err(DircolorsError::ColorParseError);
        }

        let r = u8::from_str_radix(&hex_str[0..2], 16)?;
        let g = u8::from_str_radix(&hex_str[2..4], 16)?;
        let b = u8::from_str_radix(&hex_str[4..6], 16)?;

        Ok(Color { r, g, b })
    }
}

struct Theme {
    colors: HashMap<String, Color>,
    categories: Yaml,
}

impl Theme {
    fn from_file(path: &Path) -> Result<Theme> {
        let contents = load_yaml_file(path)?;
        Self::from_string(&contents)
    }

    fn from_string(contents: &str) -> Result<Theme> {
        let mut docs = YamlLoader::load_from_str(&contents)?;
        let doc = docs.pop().expect("YAML file with one document"); // TODO

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
        })
    }

    fn get_style(&self, category: &Category) -> Result<String> {
        if category.is_empty() {
            // TODO: raise error
        }

        let mut item = &self.categories;
        for key in category {
            if let Yaml::Hash(map) = item {
                if map.contains_key(&Yaml::String("foreground".into()))
                    || map.contains_key(&Yaml::String("background".into()))
                    || map.contains_key(&Yaml::String("font-style".into()))
                {
                    break;
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

            let mut style: String = format!(
                "{font_style};38;2;{r};{g};{b}",
                font_style = *font_style_ansi,
                r = foreground.r,
                g = foreground.g,
                b = foreground.b
            );

            if let Some(background) = background {
                style.push_str(&format!(
                    ";48;2;{r};{g};{b}",
                    r = background.r,
                    g = background.g,
                    b = background.b
                ));
            }

            Ok(style)
        } else {
            Err(DircolorsError::UnexpectedYamlType)
        }
    }
}

fn run() -> Result<()> {
    let path = Path::new("filetypes.yml");
    let mapping = FileTypesMapping::from_file(&path)?;

    let theme_path = Path::new("themes/molokai.yml");
    let theme = Theme::from_file(&theme_path)?;

    let mut filetypes = mapping.mapping.keys().collect::<Vec<_>>();
    filetypes.sort_unstable_by_key(|entry| entry.len());

    let mut ls_colors: Vec<String> = vec![];
    for filetype in filetypes {
        let category = mapping.mapping.get(filetype).unwrap();
        ls_colors.push(format!("{}={}", filetype, theme.get_style(category)?).into());
    }

    println!("{}", ls_colors.join(":"));
    Ok(())
}

fn main() {
    let res = run();
    match res {
        Ok(()) => {}
        Err(e) => {
            eprintln!("Error: {}", e.description());
        }
    }
}
