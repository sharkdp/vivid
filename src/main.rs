mod color;
mod error;
mod filetypes;
mod theme;
mod types;
mod util;

use std::env;
use std::path::{Path, PathBuf};
use std::process;

use clap::{
    crate_description, crate_name, crate_version, App, AppSettings, Arg, ArgMatches, SubCommand,
};

use crate::color::ColorMode;
use crate::error::{Result, VividError};
use crate::filetypes::FileTypes;
use crate::theme::Theme;

fn get_user_config_path() -> PathBuf {
    let mut user_config_path = PathBuf::new();
    user_config_path.push(env::var("HOME").expect("Environment variable HOME"));
    user_config_path.push(".config");
    user_config_path.push("vivid");
    user_config_path
}

fn load_filetypes_database(matches: &ArgMatches, user_config_path: &PathBuf) -> Result<FileTypes> {
    let database_path_from_arg = matches.value_of("database").map(Path::new);

    let mut database_path_user = user_config_path.clone();
    database_path_user.push("filetypes.yml");

    let database_path_env_s = env::var("VIVID_DATABASE").ok();
    let database_path_env = database_path_env_s.as_ref().map(Path::new);

    let database_path_system = Path::new("/usr/share/vivid/filetypes.yml");

    let database_path = database_path_from_arg
        .or(database_path_env)
        .or_else(|| util::get_first_existing_path(&[&database_path_user, database_path_system]))
        .ok_or(VividError::CouldNotFindDatabase)?;
    FileTypes::from_file(&database_path)
}

fn load_theme(
    sub_matches: &ArgMatches,
    user_config_path: &PathBuf,
    color_mode: ColorMode,
) -> Result<Theme> {
    let theme_from_env = env::var("VIVID_THEME").ok();
    let theme = sub_matches
        .value_of("theme")
        .or_else(|| theme_from_env.as_ref().map(String::as_str))
        .unwrap_or("molokai");

    let theme_as_path = Path::new(theme);

    let theme_file = format!("{}.yml", theme);

    let mut theme_path_user = user_config_path.clone();
    theme_path_user.push("themes");
    theme_path_user.push(theme_file.clone());

    let mut theme_path_system = PathBuf::new();
    theme_path_system.push("/usr/share/vivid/themes/");
    theme_path_system.push(theme_file);

    let theme_path =
        util::get_first_existing_path(&[&theme_as_path, &theme_path_user, &theme_path_system])
            .ok_or_else(|| VividError::CouldNotFindTheme(theme.to_string()))?;
    Theme::from_file(theme_path, color_mode)
}

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
        .arg(
            Arg::with_name("color-mode")
                .long("color-mode")
                .short("m")
                .takes_value(true)
                .value_name("mode")
                .possible_values(&["8-bit", "24-bit"])
                .default_value("24-bit")
                .help("Type of ANSI colors to be used"),
        )
        .arg(
            Arg::with_name("database")
                .long("database")
                .short("d")
                .takes_value(true)
                .value_name("path")
                .help("Path to filetypes database (filetypes.yml)"),
        )
        .subcommand(
            SubCommand::with_name("generate")
                .about("Generate a LS_COLORS expression")
                .arg(Arg::with_name("theme").help("Name of the color theme")),
        )
        .subcommand(
            SubCommand::with_name("preview")
                .about("Preview a given theme")
                .arg(Arg::with_name("theme").help("Name of the color theme")),
        );

    let matches = app.get_matches();
    let color_mode = match matches.value_of("color-mode") {
        Some("8-bit") => ColorMode::BitDepth8,
        _ => ColorMode::BitDepth24,
    };

    let user_config_path = get_user_config_path();

    let filetypes = load_filetypes_database(&matches, &user_config_path)?;

    if let Some(sub_matches) = matches.subcommand_matches("generate") {
        let theme = load_theme(&sub_matches, &user_config_path, color_mode)?;

        let mut filetypes_list = filetypes.mapping.keys().collect::<Vec<_>>();
        filetypes_list.sort_unstable_by_key(|entry| entry.len());

        let mut ls_colors: Vec<String> = vec![];
        for filetype in filetypes_list {
            let category = &filetypes.mapping[filetype];
            ls_colors.push(format!("{}={}", filetype, theme.get_style(&category)?));
        }

        println!("{}", ls_colors.join(":"));
    } else if let Some(sub_matches) = matches.subcommand_matches("preview") {
        let theme = load_theme(&sub_matches, &user_config_path, color_mode)?;

        let mut pairs = filetypes.mapping.iter().collect::<Vec<_>>();
        pairs.sort_by_key(|(_, category)| category.clone());

        for (entry, category) in pairs {
            let ansi_code = theme.get_style(&category).unwrap_or("0".into());
            println!(
                "{}: \x1b[{}m{}\x1b[0m",
                category.join("."),
                ansi_code,
                entry
            );
        }
    }
    Ok(())
}

fn main() {
    let res = run();
    match res {
        Ok(()) => {}
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    }
}
