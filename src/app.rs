use std::{fs::File, io::BufReader, path::{Path, PathBuf}, ffi::OsStr};

use crate::save::{Error, Save, SaveInfo};

pub struct App {
    save: Option<SaveInfo>,
    state: EditorState,
    status_message: String,
}

struct EditorState {
    selected_save_index: usize,
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        App {
            save: None,
            state: EditorState::default(),
            status_message: String::from("working 100% perfectly for sure"),
        }
    }

    fn try_load_save<P: AsRef<Path>>(&mut self, path: P) -> Result<(), Error> {
        let file = File::open(path.as_ref())?;
        let reader = BufReader::new(file);
        let save = Save::new(reader);
        let save_info = save.load_campaign()?;
        self.save = Some(save_info);
        Ok(())
    }

    fn render_menubar(&mut self, ui: &mut egui::Ui) {
        egui::menu::bar(ui, |ui| {
            // open file
            if ui.button("Open").clicked() {
                if let Some(path) = rfd::FileDialog::new()
                    .add_filter("save", &["sav", "bak"])
                    .pick_file()
                {
                    if let Err(e) = self.try_load_save(path) {
                        let message = format!("unable to load save due to {e}");
                        eprintln!("{}", message);
                        self.status_message = message;
                    }
                }
            }
            // save file
            if ui
                .add_enabled(self.save.is_some(), egui::Button::new("Save"))
                .clicked()
            {
                if let Some(path) = rfd::FileDialog::new()
                    .add_filter("save", &["sav"])
                    .save_file()
                {
                    if let Err(e) = self.save.as_ref().unwrap().save_to_file(path) {
                        let message = format!("unable to save file due to {e}");
                        eprintln!("{}", message);
                        self.status_message = message;
                    }
                }
            }
            if ui.button("TEMP: get game path").clicked() {
                // if let Some(game_path) = rfd::FileDialog::new().set_title("Select containing game directory").pick_folder() {
                //     dbg!(&game_path);

                // }
                if let Some(game_path) = Self::get_game_directory() {
                    dbg!(&game_path);
                } else {
                    self.status_message = String::from("no game path found");
                }
            }
        });
    }

    fn get_game_directory() -> Option<PathBuf> {
        if let Some(path) = rfd::FileDialog::new().set_title("Select containing game directory").pick_folder() {
            if !path.read_dir().unwrap().any(|x| {
                x.unwrap().file_name() == OsStr::new("Magicka.exe")
            }) {
                None
            } else {
                Some(path)
            }
        } else {
            None
        }
    }

    fn render_editor(&mut self, ui: &mut egui::Ui) {
        
        if let Some(save) = &mut self.save {
            ui.horizontal(|ui| {
                for i in 0..save.get_slots().len() {
                    ui.radio_value(
                        &mut self.state.selected_save_index,
                        i,
                        format!("Save Slot {}", i + 1),
                    );
                }
            });
            let slot = save.get_slot_mut(self.state.selected_save_index);
            egui::Grid::new("editorgrid").striped(true).spacing([30.0, 4.0]).show(ui, |ui| {
                ui.label("Current Playtime:");
                ui.add(egui::DragValue::new(slot.get_current_playtime_mut()));
                ui.end_row();
                ui.label("Total Playtime:");
                ui.add(egui::DragValue::new(slot.get_total_playtime_mut()));
                ui.end_row();
            });
            ui.heading("Players:");
            for (name, data) in slot.get_players() {
                ui.label(format!("Name: {name}"));
                ui.label(format!("Data: {data:#?}"));
            }
        } else {
            ui.heading("No save loaded");
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("menubar").show(ctx, |ui| {
            self.render_menubar(ui);
        });

        egui::TopBottomPanel::bottom("statusbar").show(ctx, |ui| {
            ui.label(format!("status: {}", self.status_message));
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            self.render_editor(ui);
        });
    }
}

impl Default for EditorState {
    fn default() -> Self {
        Self {
            selected_save_index: 0,
        }
    }
}
