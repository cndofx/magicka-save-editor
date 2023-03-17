use std::path::PathBuf;

use iced::{Sandbox, widget::{button, row, text, container}, Length};
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
            steam_dir: SteamDir::locate().unwrap(),
            game_path: None,
        }
    }

    fn title(&self) -> String {
        String::from("Counter")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::DetectGamePathClicked => {
                match self.steam_dir.app(&MAGICKA_APP_ID) {
                    Some(app) => {
                        println!("found magicka at {:?}", app.path);
                        self.game_path = Some(app.path.clone());
                    },
                    None => {
                        println!("magicka not found");
                    },
                }
            }
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        let detect_path_button = button("Detect Game Path").on_press(Message::DetectGamePathClicked);
        let path_text = if let Some(path) = &self.game_path {
            text(path.display())
        } else {
            text("Game path not set")
        };

        let content = row![
            detect_path_button,
            path_text,
        ]
        .spacing(20)
        .padding(5);

        content.into()

        // container(content).width(Length::Fill).height(Length::Fill).center_x().center_y().into()
    }

    fn theme(&self) -> iced::Theme {
        iced::Theme::Dark
    }
}