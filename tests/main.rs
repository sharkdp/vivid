use assert_cmd::Command;

#[test]
fn can_call_vivid_generate_for_all_themes() {
    let themes_dir = std::fs::read_dir("themes").unwrap();
    for theme in themes_dir {
        let theme = theme.unwrap();
        let theme_path = theme.path();
        let theme_name = theme_path
            .file_stem()
            .unwrap()
            .to_string_lossy()
            .replace(".yml", "");

        let mut cmd = Command::cargo_bin("vivid").unwrap();

        cmd.arg("generate").arg(theme_name).assert().success();
    }
}
