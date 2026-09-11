use std::collections::HashMap;
use std::path::Path;

use rust_embed::RustEmbed;
use yaml_rust::yaml::YamlLoader;
use yaml_rust::Yaml;

use crate::error::{Result, VividError};
use crate::types::{Category, FileType};
use crate::util::load_yaml_file;

pub struct FileTypes {
    pub mapping: HashMap<FileType, Category>,
}

#[derive(RustEmbed)]
#[folder = "config/"]
struct ConfigAssets;

impl FileTypes {
    pub fn from_path(path: &Path) -> Result<FileTypes> {
        let contents = load_yaml_file(path)
            .map_err(|_| VividError::CouldNotLoadDatabaseFrom(path.to_string_lossy().into()))?;
        Self::from_string(&contents)
    }

    pub fn from_embedded() -> Result<FileTypes> {
        let filetypes = ConfigAssets::get("filetypes.yml").unwrap();

        let contents = std::str::from_utf8(&filetypes.data)
            .map_err(|_| VividError::CouldNotLoadDatabaseFrom(String::from("embedded defaults")))?;
        Self::from_string(contents)
    }

    fn from_string(contents: &str) -> Result<FileTypes> {
        let docs = YamlLoader::load_from_str(contents)?;
        let doc = &docs[0];

        // A database can start from the bundled defaults by adding an `include: default`
        // key at the top level. The remaining entries are then treated as additions or
        // overrides on top of those defaults, so users only need to list what they want
        // to change instead of copying the whole database.
        if let Yaml::Hash(hash) = doc {
            let include_key = Yaml::String("include".to_string());
            if let Some(include) = hash.get(&include_key) {
                let mut base = Self::included_base(include)?;

                let mut overrides = hash.clone();
                overrides.remove(&include_key);
                let extra = Self::get_mapping(&Yaml::Hash(overrides), &vec![])?;
                base.mapping.extend(extra.mapping);

                return Ok(base);
            }
        }

        Self::get_mapping(doc, &vec![])
    }

    fn included_base(include: &Yaml) -> Result<FileTypes> {
        match include {
            Yaml::String(name) if name == "default" => Self::from_embedded(),
            Yaml::String(name) => Err(VividError::UnknownInclude(name.clone())),
            _ => Err(VividError::UnexpectedYamlType),
        }
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

    fn get_mapping(value: &Yaml, category: &Category) -> Result<FileTypes> {
        let mut mapping = HashMap::new();

        match value {
            Yaml::Array(array) => {
                for filetype in array {
                    if let Yaml::String(filetype) = filetype {
                        let code = Self::get_code(filetype);
                        let result = mapping.insert(code, category.clone());

                        if result.is_some() {
                            return Err(VividError::DuplicateFileType(filetype.to_string()));
                        }
                    } else {
                        return Err(VividError::UnexpectedYamlType);
                    }
                }
            }
            Yaml::Hash(ref map) => {
                for (key, value) in map {
                    let mut child_category = category.clone();
                    if let Yaml::String(key) = key {
                        child_category.push(key.clone());
                    }
                    let child_mapping = Self::get_mapping(value, &child_category)?;

                    if let Some(filetype) = child_mapping
                        .mapping
                        .keys()
                        .find(|ft| mapping.contains_key(*ft))
                    {
                        return Err(VividError::DuplicateFileType(filetype.to_string()));
                    }

                    mapping.extend(child_mapping.mapping);
                }
            }
            _ => {
                return Err(VividError::UnexpectedYamlType);
            }
        }

        Ok(FileTypes { mapping })
    }
}

#[cfg(test)]
mod tests {
    use super::FileTypes;

    #[test]
    fn basic() {
        let ft = FileTypes::from_string(
            "
                core:
                  - .ext1

                bar:
                  baz: [.ext2, .ext3]
            ",
        )
        .unwrap();

        assert_eq!(vec!["core".to_string()], ft.mapping["*.ext1"]);
        assert_eq!(
            vec!["bar".to_string(), "baz".to_string()],
            ft.mapping["*.ext2"]
        );
        assert_eq!(
            vec!["bar".to_string(), "baz".to_string()],
            ft.mapping["*.ext3"]
        );
    }

    #[test]
    fn include_default_overrides_existing_extension() {
        let ft = FileTypes::from_string(
            "
                include: default

                programming:
                  source:
                    objective_c: [.m]
            ",
        )
        .unwrap();

        // `.m` now resolves to Objective-C instead of the default Matlab mapping.
        assert_eq!(
            vec![
                "programming".to_string(),
                "source".to_string(),
                "objective_c".to_string()
            ],
            ft.mapping["*.m"]
        );

        // Extensions that were not touched keep their default categories.
        assert_eq!(
            vec![
                "programming".to_string(),
                "source".to_string(),
                "rust".to_string()
            ],
            ft.mapping["*.rs"]
        );
    }

    #[test]
    fn include_default_adds_new_extension() {
        let ft = FileTypes::from_string(
            "
                include: default

                programming:
                  source:
                    objective_c: [.mm]
            ",
        )
        .unwrap();

        assert!(ft.mapping.contains_key("*.mm"));
        assert!(ft.mapping.contains_key("*.rs"));
    }

    #[test]
    fn unknown_include_target_is_an_error() {
        assert!(FileTypes::from_string("include: nonsense\n").is_err());
    }
}
