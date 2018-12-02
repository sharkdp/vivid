use std::error::Error;
use std::fmt::Display;
use std::io;
use std::result;

use yaml_rust::ScanError;

#[derive(Debug)]
pub enum VividError {
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

impl Display for VividError {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> result::Result<(), std::fmt::Error> {
        match self {
            VividError::IoError(e) => write!(fmt, "{}", e.description()),
            VividError::YamlParsingError(e) => write!(fmt, "{}", e.description()),
            VividError::UnexpectedYamlType => write!(fmt, "Unexpected type in YAML file."),
            VividError::ColorParseError(color_str) => {
                write!(fmt, "Could not parse color string '{}'.", color_str)
            }
            VividError::DuplicateFileType(ft) => write!(fmt, "Duplicate file type '{}'.", ft),
            VividError::CouldNotLoadTheme(path) => write!(fmt, "Could not load theme '{}'.", path),
            VividError::EmptyThemeFile => write!(fmt, "Theme file is empty"),
            VividError::CouldNotFindStyleFor(category) => {
                write!(fmt, "Could not find style for category '{}'", category)
            }
            VividError::UnknownColor(color) => write!(fmt, "Unknown color '{}'", color),
        }
    }
}

impl Error for VividError {
    fn description(&self) -> &str {
        "Dummy implementation: use .fmt()"
    }
}

impl From<io::Error> for VividError {
    fn from(e: io::Error) -> Self {
        VividError::IoError(e)
    }
}

impl From<ScanError> for VividError {
    fn from(e: ScanError) -> Self {
        VividError::YamlParsingError(e)
    }
}

pub type Result<T> = std::result::Result<T, VividError>;
