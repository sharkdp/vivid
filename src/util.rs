use std::fs::File;
use std::io::Read;
use std::path::Path;

use crate::error::Result;

pub fn load_yaml_file(path: &Path) -> Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

/// Helper function that might appear in Rust stable at some point
/// (https://doc.rust-lang.org/stable/std/option/enum.Option.html#method.transpose)
pub fn transpose<T, E>(
    opt: Option<std::result::Result<T, E>>,
) -> std::result::Result<Option<T>, E> {
    opt.map_or(Ok(None), |res| res.map(Some))
}

pub fn get_first_existing_path<'a>(paths: &[&'a Path]) -> Option<&'a Path> {
    paths.iter().find(|p| Path::exists(*p)).copied()
}

pub fn get_all_existing_paths<'a>(paths: &[&'a Path]) -> Vec<&'a Path> {
    paths.iter().cloned().filter(|p| Path::exists(*p)).collect()
}
