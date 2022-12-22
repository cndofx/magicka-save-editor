pub mod save;
pub mod app;

pub const CONFY_APP_NAME: &str = "magicka-save-editor";
pub const CONFY_CONFIG_NAME: Option<&str> = Some("config");

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Config {
    pub game_directory: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            game_directory: None,
        }
    }
}