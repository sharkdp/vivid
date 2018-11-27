extern crate yaml_rust;
#[macro_use]
extern crate clap;
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

use clap::{App, AppSettings, Arg, SubCommand};

use error::Result;
use filetypes::FileTypes;
use theme::Theme;

fn run() -> Result<()> {
    let app = App::new(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        .global_setting(AppSettings::ColorAuto)
        .global_setting(AppSettings::ColoredHelp)
        .global_setting(AppSettings::DeriveDisplayOrder)
        .global_setting(AppSettings::UnifiedHelpMessage)
        .setting(AppSettings::SubcommandRequired)
        .setting(AppSettings::InferSubcommands)
        .setting(AppSettings::VersionlessSubcommands)
        .max_term_width(100)
        .subcommand(
            SubCommand::with_name("generate")
                .about("Generate a LS_COLORS expression")
                .arg(
                    Arg::with_name("filetypes-db")
                        .required(true)
                        .help("Path to file-types database (filetypes.yml)"),
                )
                .arg(
                    Arg::with_name("theme")
                        .long("theme")
                        .short("t")
                        .takes_value(true)
                        .required(true)
                        .help("Path to theme file (YML)"),
                ),
        );

    let matches = app.get_matches();

    if let Some(sub_matches) = matches.subcommand_matches("generate") {
        let path = Path::new(sub_matches.value_of("filetypes-db").unwrap());
        let filetypes = FileTypes::from_file(&path)?;

        let theme_path = Path::new(sub_matches.value_of("theme").unwrap());
        let theme = Theme::from_file(&theme_path)?;

        let mut filetypes_list = filetypes.mapping.keys().collect::<Vec<_>>();
        filetypes_list.sort_unstable_by_key(|entry| entry.len());

        let mut ls_colors: Vec<String> = vec![];
        for filetype in filetypes_list {
            let category = &filetypes.mapping[filetype];
            ls_colors.push(format!("{}={}", filetype, theme.get_style(&category)?));
        }

        println!("{}", ls_colors.join(":"));
    }
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
