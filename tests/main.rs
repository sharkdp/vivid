use assert_cmd::Command;

fn generated_style(args: &[&str], key: &str) -> String {
    let output = Command::cargo_bin("vivid")
        .unwrap()
        .args(args)
        .output()
        .unwrap();
    assert!(output.status.success());
    String::from_utf8(output.stdout)
        .unwrap()
        .trim()
        .split(':')
        .find_map(|entry| entry.strip_prefix(&format!("{key}=")).map(str::to_owned))
        .unwrap()
}

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

#[test]
fn italic_styles_require_flag() {
    assert_eq!("0;36", generated_style(&["generate", "ansi"], "ln"));
    assert_eq!(
        "3;36",
        generated_style(&["--italic", "generate", "ansi"], "ln")
    );
    assert_eq!(
        "3;36",
        generated_style(&["generate", "ansi", "--italic"], "ln")
    );
}
