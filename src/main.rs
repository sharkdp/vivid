extern crate yaml_rust;
#[macro_use]
extern crate lazy_static;

mod color;
mod error;
mod filetypes;
mod theme;
mod types;
mod util;

use std::error::Error;
use std::path::Path;

use error::Result;
use filetypes::FileTypes;
use theme::Theme;

fn run() -> Result<()> {
    let path = Path::new("filetypes.yml");
    let filetypes = FileTypes::from_file(&path)?;

    let theme_path = Path::new("themes/molokai.yml");
    let theme = Theme::from_file(&theme_path)?;

    let mut filetypes_list = filetypes.mapping.keys().collect::<Vec<_>>();
    filetypes_list.sort_unstable_by_key(|entry| entry.len());

    let mut ls_colors: Vec<String> = vec![];
    for filetype in filetypes_list {
        let category = &filetypes.mapping[filetype];
        ls_colors.push(format!("{}={}", filetype, theme.get_style(&category)?));
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
