use std::collections::HashMap;
use std::path::Path;

use yaml_rust::yaml::YamlLoader;
use yaml_rust::Yaml;
use rust_embed::RustEmbed;

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

        let contents = std::str::from_utf8(filetypes.as_ref())
            .or_else(Err(VividError::CouldNotLoadDatabaseFrom(String::from("embedded file"))))?;
        Self::from_string(contents)
    }

    fn from_string(contents: &str) -> Result<FileTypes> {
        let docs = YamlLoader::load_from_str(&contents)?;
        let doc = &docs[0];

        Self::get_mapping(doc, &vec![])
    }

    fn get_code(filetype: &str) -> String {
        if filetype.get(0..1) == Some("$") {
            filetype[1..].into()
        } else {
            let mut s = String::from("*");
            s.push_str(&filetype);
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
}
