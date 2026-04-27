use rust_embed::RustEmbed;

use crate::error::{Result, VividError};

/*
    Naming convention: completions/[shell name]
    For example: completions/bash
*/
#[derive(RustEmbed)]
#[folder = "completions/"]
struct CompletionAssets;

pub fn get_completion_as_str(sh: &str) -> Result<String> {
    let completion_file = CompletionAssets::get(sh)
        .ok_or_else(|| VividError::CouldNotFindCompletionFile(sh.to_string()))?;
    let contents = std::str::from_utf8(&completion_file.data).map_err(|_| {
        VividError::IoError(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            format!("Completion file '{}' is not valid UTF-8.", sh),
        ))
    })?;
    Ok(contents.to_string())
}

pub fn get_available_completion_files() -> Vec<String> {
    CompletionAssets::iter()
        .map(|path| path.into_owned())
        .collect()
}
