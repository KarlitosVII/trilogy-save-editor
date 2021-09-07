use std::fs;

use regex::Regex;

fn main() {
    // app/Cargo.toml
    const APP_CARGO_PATH: &str = "app/Cargo.toml";
    let mut app_cargo = fs::read_to_string(APP_CARGO_PATH).expect(APP_CARGO_PATH);

    // version = "*"
    let regex = Regex::new(r#"(?m)^\s*version\s*=\s*"([\d.]+)"\s*$"#).unwrap();
    let captures = regex.captures(&app_cargo).expect("regex doesn't match");
    if &captures[1] != env!("CARGO_PKG_VERSION") {
        let range = captures.get(1).unwrap().range();
        app_cargo.replace_range(range, env!("CARGO_PKG_VERSION"));
        fs::write(APP_CARGO_PATH, app_cargo).unwrap();
    }

    // InnoSetup.iss
    const INNO_SETUP_PATH: &str = "InnoSetup.iss";
    let mut inno_setup = fs::read_to_string(INNO_SETUP_PATH).expect(INNO_SETUP_PATH);

    // #define AppVersion "*"
    let regex = Regex::new(r#"(?m)^\s*#define\s+AppVersion\s+"([\d.]+)"\s*$"#).unwrap();
    let captures = regex.captures(&inno_setup).expect("regex doesn't match");
    if &captures[1] != env!("CARGO_PKG_VERSION") {
        let range = captures.get(1).unwrap().range();
        inno_setup.replace_range(range, env!("CARGO_PKG_VERSION"));
        fs::write(INNO_SETUP_PATH, inno_setup).unwrap();
    }

    println!("cargo:rerun-if-env-changed=CARGO_PKG_VERSION");
}
