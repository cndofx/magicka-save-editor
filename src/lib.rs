pub mod save;
pub mod app;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Config {
    pub game_directory: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            game_directory: "".into(),
        }
    }
}