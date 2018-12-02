use std::error::Error;
use std::fmt::Display;
use std::io;
use std::result;

use yaml_rust::ScanError;

#[derive(Debug)]
pub enum DircolorsError {
    IoError(io::Error),
    YamlParsingError(ScanError),
    UnexpectedYamlType,
    ColorParseError(String),
    DuplicateFileType(String),
    CouldNotLoadTheme(String),
    EmptyThemeFile,
    CouldNotFindStyleFor(String),
    UnknownColor(String),
}

impl Display for DircolorsError {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> result::Result<(), std::fmt::Error> {
        match self {
            DircolorsError::IoError(e) => write!(fmt, "{}", e.description()),
            DircolorsError::YamlParsingError(e) => write!(fmt, "{}", e.description()),
            DircolorsError::UnexpectedYamlType => write!(fmt, "Unexpected type in YAML file."),
            DircolorsError::ColorParseError(color_str) => {
                write!(fmt, "Could not parse color string '{}'.", color_str)
            }
            DircolorsError::DuplicateFileType(ft) => write!(fmt, "Duplicate file type '{}'.", ft),
            DircolorsError::CouldNotLoadTheme(path) => {
                write!(fmt, "Could not load theme '{}'.", path)
            }
            DircolorsError::EmptyThemeFile => write!(fmt, "Theme file is empty"),
            DircolorsError::CouldNotFindStyleFor(category) => {
                write!(fmt, "Could not find style for category '{}'", category)
            }
            DircolorsError::UnknownColor(color) => write!(fmt, "Unknown color '{}'", color),
        }
    }
}

impl Error for DircolorsError {
    fn description(&self) -> &str {
        "Dummy implementation: use .fmt()"
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

pub type Result<T> = std::result::Result<T, DircolorsError>;
