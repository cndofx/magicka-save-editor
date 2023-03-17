use std::path::PathBuf;

use iced::{
    widget::{button, column, container, horizontal_rule, row, text},
    Sandbox, Alignment,
};
use steamlocate::SteamDir;

const MAGICKA_APP_ID: u32 = 42910;

pub struct App {
    steam_dir: SteamDir,
    game_path: Option<PathBuf>,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    DetectGamePathClicked,
}

impl Sandbox for App {
    type Message = Message;

    fn new() -> Self {
        App {
            // todo: gracefully handle situation when steamdir cant be found
            steam_dir: SteamDir::locate().unwrap(),
            game_path: None,
        }
    }

    fn title(&self) -> String {
        String::from("Counter")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::DetectGamePathClicked => match self.steam_dir.app(&MAGICKA_APP_ID) {
                Some(app) => {
                    self.game_path = Some(app.path.clone());
                }
                None => {
                    eprintln!("magicka directory not found");
                }
            },
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        let detect_path_button =
            button("Detect Game Path").on_press(Message::DetectGamePathClicked);
        let path_text = if let Some(path) = &self.game_path {
            text(path.display())
        } else {
            text("Game path not set")
        };

        let row = row![detect_path_button, path_text,].spacing(20).align_items(Alignment::Center);

        let content = column![row, horizontal_rule(15), text("test"),];

        container(content).padding(5).into()
    }

    fn theme(&self) -> iced::Theme {
        iced::Theme::Dark
    }
}
