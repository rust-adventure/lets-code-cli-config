use std::fs;

use directories::ProjectDirs;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Config {
    name: String,
    port: Option<u16>,
}
fn main() {
    if let Some(proj_dirs) = ProjectDirs::from(
        "dev",
        "rust-adventure",
        "cli-config",
    ) {
        let config_dir = proj_dirs.config_dir();

        let config_file = fs::read_to_string(
            config_dir.join("mycli.toml"),
        );

        let config: Config = match config_file {
            Ok(file) => toml::from_str(&file).unwrap(),
            Err(_) => Config {
                name: "Chris Biscardi".to_string(),
                port: Some(4000),
            },
        };

        dbg!(config);

        // Linux:   /home/alice/.config/barapp
        // Windows: C:\Users\Alice\AppData\Roaming\Foo Corp\Bar App
        // macOS:   /Users/Alice/Library/Application Support/com.Foo-Corp.Bar-App
    }
}
