use crate::save::SaveInfo;

pub struct App {
    save: Option<SaveInfo>,
    state: EditorState,
}

struct EditorState {

}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>, text: &str) -> Self {
        App {
            save: None,
            state: EditorState::default(),
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("hello world");
        });
    }
}

impl Default for EditorState {
    fn default() -> Self {
        Self {  }
    }
}