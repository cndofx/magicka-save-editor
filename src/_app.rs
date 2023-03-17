use std::{
    ffi::OsStr,
    fs::File,
    io::BufReader,
    path::{Path, PathBuf},
};

use crate::{
    save::{Error, Save, SaveInfo},
    Config, CONFY_APP_NAME, CONFY_CONFIG_NAME,
};

pub struct App {
    save: Option<SaveInfo>,
    state: EditorState,
    config: Config,
    game_directory_set: bool,
}

struct EditorState {
    selected_save_index: usize,
    staves: Option<Vec<String>>,
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>, config: Config) -> Self {
        let game_directory_set = config.game_directory.is_some();
        App {
            save: None,
            state: EditorState::default(),
            config,
            game_directory_set,
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

    fn get_game_directory() -> Option<PathBuf> {
        if let Some(path) = rfd::FileDialog::new()
            .set_title("Select containing game directory")
            .pick_folder()
        {
            if !path
                .read_dir()
                .unwrap()
                .any(|x| x.unwrap().file_name() == OsStr::new("Magicka.exe"))
            {
                None
            } else {
                Some(path)
            }
        } else {
            None
        }
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
                    }
                }
            }
        });
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
            egui::Grid::new("editorgrid")
                .striped(true)
                .spacing([30.0, 4.0])
                .show(ui, |ui| {
                    ui.label("Current Playtime:");
                    ui.add(egui::DragValue::new(slot.get_current_playtime_mut()));
                    ui.end_row();
                    ui.label("Total Playtime:");
                    ui.add(egui::DragValue::new(slot.get_total_playtime_mut()));
                    ui.end_row();
                    ui.label("Current Level:");
                    ui.add(egui::DragValue::new(slot.get_current_level_mut()));
                    ui.end_row();
                    ui.label("Max Allowed Level:");
                    ui.add(egui::DragValue::new(slot.get_max_allowed_level_mut()));
                    ui.end_row();
                    ui.label("Looped (NG+):");
                    ui.checkbox(slot.get_looped_mut(), "");
                    ui.end_row();
                });
            egui::CollapsingHeader::new("Players")
                .default_open(true)
                .show(ui, |ui| {
                    if !self.game_directory_set {
                        ui.heading("You must set the Magicka game directory before use");
                    }
                    ui.add_enabled_ui(self.game_directory_set, |ui| {
                        egui::Grid::new("editorplayersgrid")
                            .spacing([15.0, 4.0])
                            .min_col_width(100.0)
                            .max_col_width(250.0)
                            .show(ui, |ui| {
                                for player in slot.get_players_mut() {
                                    ui.label("Name");
                                    ui.label("Staff");
                                    ui.label("Weapon");
                                    ui.end_row();
                                    ui.add(
                                        egui::TextEdit::singleline(player.get_name_mut())
                                            .desired_width(150.0),
                                    );
                                    ui.add(
                                        egui::TextEdit::singleline(player.get_staff_mut())
                                            .desired_width(150.0),
                                    );
                                    ui.add(
                                        egui::TextEdit::singleline(player.get_weapon_mut())
                                            .desired_width(150.0),
                                    );
                                    ui.end_row();
                                }
                            });
                    });
                });
            ui.heading("Players:");
            for player in slot.get_players_mut() {
                ui.label(format!("Name: {}", player.get_name_mut()));
                ui.label(format!("Staff: {}", player.get_staff_mut()));
                ui.label(format!("Weapon: {}", player.get_weapon_mut()));
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

        egui::TopBottomPanel::bottom("statusbar")
            .exact_height(40.0)
            .show(ctx, |ui| {
                ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                    if ui.button("Set Game Directory").clicked() {
                        if let Some(directory) = Self::get_game_directory() {
                            self.config.game_directory = directory.to_str().map(|s| s.to_string());
                            confy::store(CONFY_APP_NAME, CONFY_CONFIG_NAME, &self.config).unwrap();
                        }
                    }
                    ui.add_space(20.0);
                    if let Some(game_directory) = &self.config.game_directory {
                        ui.label(format!("Using game directory: '{game_directory}'"));
                    } else {
                        ui.horizontal(|ui| {
                            ui.label("No game directory loaded.");
                        });
                    }
                });
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
            staves: None,
        }
    }
}