use std::error::Error;
use std::fmt::Display;
use std::io;
use std::num::ParseIntError;
use std::result;

use yaml_rust::ScanError;

#[derive(Debug)]
pub enum DircolorsError {
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
    fn from(_e: ParseIntError) -> Self {
        DircolorsError::ColorParseError
    }
}

pub type Result<T> = std::result::Result<T, DircolorsError>;
