use std::fs::File;
use std::io::Read;
use std::path::Path;

use error::Result;

pub fn load_yaml_file(path: &Path) -> Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
