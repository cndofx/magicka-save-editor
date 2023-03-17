use std::{
    fs::File,
    io::BufReader,
    path::{Path, PathBuf},
};

use iced::{
    widget::{button, column, container, horizontal_rule, radio, row, text},
    Alignment, Sandbox,
};
use steamlocate::SteamDir;

use crate::save::{Error, Save, SaveInfo};

const MAGICKA_APP_ID: u32 = 42910;

pub struct App {
    steam_dir: SteamDir,
    game_path: Option<PathBuf>,
    save_path: Option<PathBuf>,

    save_info: Option<SaveInfo>,
    selected_save_slot: usize,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    DetectGamePathClicked,
    OpenFileClicked,
    SelectedSlotChanged(usize),
}

impl Sandbox for App {
    type Message = Message;

    fn new() -> Self {
        App {
            // todo: gracefully handle situation when steamdir cant be found
            steam_dir: SteamDir::locate().unwrap(),
            game_path: None,
            save_path: None,
            save_info: None,
            selected_save_slot: 0,
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

                    let mut save_path = app.path.clone();
                    save_path.push("SaveData");
                    self.save_path = Some(save_path);
                }
                None => {
                    eprintln!("magicka directory not found");
                }
            },
            Message::OpenFileClicked => {
                if let Some(path) = rfd::FileDialog::new()
                    .add_filter("save", &["sav", "bak"])
                    .set_directory(self.save_path.as_ref().unwrap())
                    .pick_file()
                {
                    if let Err(e) = self.try_load_save(path) {
                        let message = format!("unable to load save due to {e}");
                        eprintln!("{}", message);
                    }
                }
            }
            Message::SelectedSlotChanged(value) => {
                self.selected_save_slot = value;
            }
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        let game_detector = self.view_game_detector();
        
        let saves_panel = self.view_saves_panel();

        let content = column![game_detector, horizontal_rule(15), saves_panel];

        container(content).padding(5).into()
    }

    fn theme(&self) -> iced::Theme {
        iced::Theme::Dark
    }
}

impl App {
    fn view_game_detector(&self) -> iced::Element<Message> {
        let detect_path_button =
            button("Detect Game Path").on_press(Message::DetectGamePathClicked);
        let path_text = if let Some(path) = &self.game_path {
            text(path.display())
        } else {
            text("Game path not set")
        };

        row![detect_path_button, path_text,]
            .spacing(20)
            .align_items(Alignment::Center)
            .into()
    }

    fn view_saves_panel(&self) -> iced::Element<Message> {
        let open_button = button("Open Save").on_press(Message::OpenFileClicked);

        let slots: iced::Element<Message> = if let Some(save_info) = &self.save_info {
            let choose_save_slot =
                (0..save_info.get_slots().len()).fold(row![].spacing(10), |row, slot_idx| {
                    row.push(radio(
                        format!("Slot {}", slot_idx + 1),
                        slot_idx,
                        Some(self.selected_save_slot),
                        Message::SelectedSlotChanged,
                    ))
                });
            choose_save_slot.into()
        } else {
            text("No save loaded").into()
        };

        row![open_button, slots].spacing(10).align_items(Alignment::Center).into()
    }

    fn try_load_save(&mut self, path: impl AsRef<Path>) -> Result<(), Error> {
        let file = File::open(path.as_ref())?;
        let reader = BufReader::new(file);
        let save = Save::new(reader);
        let save_info = save.load_campaign()?;
        self.save_info = Some(save_info);
        Ok(())
    }
}
