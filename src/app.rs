use std::{path::Path, fs::File, io::BufReader};

use crate::save::{SaveInfo, Error, Save};

pub struct App {
    save: Option<SaveInfo>,
    state: EditorState,
}

struct EditorState {}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>, text: &str) -> Self {
        App {
            save: None,
            state: EditorState::default(),
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
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("menubar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                if ui.button("Open").clicked() {
                    if let Some(path) = rfd::FileDialog::new().pick_file() {
                        println!("got path {}", path.display());
                        if let Err(e) = self.try_load_save(path) {
                            eprintln!("unable to load save:\n{e}");
                        }
                    }
                }
                if ui
                    .add_enabled(self.save.is_some(), egui::Button::new("Save"))
                    .clicked()
                {
                    println!("save");
                }
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            let text = format!("save loaded: {}", self.save.is_some());
            ui.label(text);
            if let Some(save) = &self.save {
                let text = format!("has {} save slots", save.get_slots().len());
                ui.label(text);
            }
        });
    }
}

impl Default for EditorState {
    fn default() -> Self {
        Self {}
    }
}
