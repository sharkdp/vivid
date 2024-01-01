mod color;
mod error;
mod filetypes;
mod font_style;
mod theme;
mod types;
mod util;

use etcetera::BaseStrategy;
use rust_embed::RustEmbed;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process;
use std::{env, fs};

use clap::{
    crate_description, crate_name, crate_version, Arg, ArgAction, ArgMatches, ColorChoice, Command,
};

use crate::color::ColorMode;
use crate::error::{Result, VividError};
use crate::filetypes::FileTypes;
use crate::theme::Theme;

#[derive(RustEmbed)]
#[folder = "themes/"]
struct ThemeAssets;

const THEME_PATH_SYSTEM: &str = "/usr/share/vivid/themes/";

fn load_filetypes_database(matches: &ArgMatches, user_config_path: &Path) -> Result<FileTypes> {
    let database_path_from_arg = matches.get_one::<String>("database").map(Path::new);

    let mut database_path_user = user_config_path.to_owned();
    database_path_user.push("filetypes.yml");

    let database_path_env_s = env::var("VIVID_DATABASE").ok();
    let database_path_env = database_path_env_s.as_ref().map(Path::new);

    let database_path_system = Path::new("/usr/share/vivid/filetypes.yml");

    let database_path = database_path_from_arg
        .or(database_path_env)
        .or_else(|| util::get_first_existing_path(&[&database_path_user, database_path_system]));

    // If there is a specified database file and it exists, use it.
    // Otherwise, use the embedded file.
    match database_path {
        Some(path) => FileTypes::from_path(path),
        None => FileTypes::from_embedded(),
    }
}

fn available_theme_names(user_config_path: &Path) -> Result<Vec<String>> {
    let theme_path_user = user_config_path.join("themes");
    let theme_path_system = PathBuf::from(THEME_PATH_SYSTEM);
    let theme_paths = util::get_all_existing_paths(&[&theme_path_user, &theme_path_system]);

    // build from default themes first
    let mut available_themes: Vec<String> = ThemeAssets::iter()
        .map(|theme_name| theme_name.trim_end_matches(".yml").to_owned())
        .collect::<Vec<_>>();

    for path in theme_paths {
        let dir = fs::read_dir(path).map_err(VividError::IoError)?;
        for theme_file in dir {
            let theme_name = theme_file
                .map_err(VividError::IoError)?
                .file_name()
                .into_string()
                .map_err(|n| {
                    VividError::InvalidFileName(n.as_os_str().to_string_lossy().into_owned())
                })?;
            available_themes.push(theme_name.trim_end_matches(".yml").to_owned());
        }
    }
    available_themes.sort();
    available_themes.dedup();
    Ok(available_themes)
}

fn load_theme(
    sub_matches: &ArgMatches,
    user_config_path: &Path,
    color_mode: ColorMode,
) -> Result<Theme> {
    let theme_from_env = env::var("VIVID_THEME").ok();
    let theme = sub_matches
        .get_one::<String>("theme")
        .map(|s| s.as_str())
        .or(theme_from_env.as_deref())
        // Convert option to result, then unwrap value or return error if None
        .ok_or_else(|| VividError::NoThemeProvided)?;

    let theme_as_path = Path::new(theme);

    let theme_file = format!("{}.yml", theme);

    let mut theme_path_user = user_config_path.to_owned();
    theme_path_user.push("themes");
    theme_path_user.push(theme_file.clone());

    let mut theme_path_system = PathBuf::new();
    theme_path_system.push(THEME_PATH_SYSTEM);
    theme_path_system.push(&theme_file);

    let theme_path =
        util::get_first_existing_path(&[theme_as_path, &theme_path_user, &theme_path_system]);

    match theme_path {
        Some(path) => return Theme::from_path(path, color_mode),
        None => {
            if let Some(embedded_file) = ThemeAssets::get(&theme_file) {
                if let Ok(embedded_data) = std::str::from_utf8(&embedded_file.data) {
                    return Theme::from_string(embedded_data, color_mode);
                }
            }
        }
    }
    Err(VividError::CouldNotFindTheme(theme.to_string()))
}

fn cli() -> clap::Command {
    Command::new(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        .color(ColorChoice::Auto)
        .subcommand_required(true)
        .infer_subcommands(true)
        .max_term_width(100)
        .arg(
            Arg::new("color-mode")
                .long("color-mode")
                .short('m')
                .action(ArgAction::Set)
                .value_name("mode")
                .value_parser(["8-bit", "24-bit"])
                .default_value("24-bit")
                .help("Type of ANSI colors to be used"),
        )
        .arg(
            Arg::new("database")
                .long("database")
                .short('d')
                .action(ArgAction::Set)
                .value_name("path")
                .help("Path to filetypes database (filetypes.yml)"),
        )
        .subcommand(
            Command::new("generate")
                .about("Generate a LS_COLORS expression")
                .arg(
                    Arg::new("theme")
                        .help("Name of the color theme")
                        .action(ArgAction::Set),
                ),
        )
        .subcommand(
            Command::new("preview").about("Preview a given theme").arg(
                Arg::new("theme")
                    .help("Name of the color theme")
                    .action(ArgAction::Set),
            ),
        )
        .subcommand(Command::new("themes").about("Prints list of available themes"))
}

fn run() -> Result<()> {
    let matches = cli().get_matches();
    let color_mode = match matches.get_one::<String>("color-mode").map(|s| s.as_str()) {
        Some("8-bit") => ColorMode::BitDepth8,
        _ => ColorMode::BitDepth24,
    };

    let basedirs = etcetera::choose_base_strategy().expect("Could not get home directory");
    let user_config_path = basedirs.config_dir().join("vivid");

    let filetypes = load_filetypes_database(&matches, &user_config_path)?;

    let stdout = io::stdout();
    let mut stdout_lock = stdout.lock();

    if let Some(sub_matches) = matches.subcommand_matches("generate") {
        let theme = load_theme(sub_matches, &user_config_path, color_mode)?;

        let mut mapping = filetypes
            .mapping
            .iter()
            .map(|(filetype, category)| (filetype, theme.get_style(category)))
            .map(|(filetype, style)| style.map(|style| (filetype, style)))
            .collect::<Result<Vec<_>>>()?;

        // Sort the keys deterministically.  Shorter keys come first so that e.g.
        // *README.md will override *.md.
        mapping.sort_unstable_by_key(|&(filetype, _)| (filetype.len(), filetype));

        let ls_colors: Vec<_> = mapping
            .iter()
            .map(|(filetype, style)| format!("{}={}", filetype, style))
            .collect();

        writeln!(stdout_lock, "{}", ls_colors.join(":")).ok();
    } else if let Some(sub_matches) = matches.subcommand_matches("preview") {
        let theme = load_theme(sub_matches, &user_config_path, color_mode)?;

        let mut pairs = filetypes.mapping.iter().collect::<Vec<_>>();
        pairs.sort_by_key(|(_, category)| *category);

        for (entry, category) in pairs {
            let ansi_code = theme.get_style(category).unwrap_or_else(|_| "0".into());
            writeln!(
                stdout_lock,
                "{}: \x1b[{}m{}\x1b[0m",
                category.join("."),
                ansi_code,
                entry
            )
            .ok();
        }
    } else if matches.subcommand_matches("themes").is_some() {
        for theme in available_theme_names(&user_config_path)? {
            writeln!(stdout_lock, "{}", theme).ok();
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

#[test]
fn verify_cli() {
    cli().debug_assert();
}
