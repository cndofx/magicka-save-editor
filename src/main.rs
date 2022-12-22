use magicka_save_editor::Config;

fn main() -> eyre::Result<()> {
    let config: Config = confy::load("magicka-save-editor", Some("config"))?;
    confy::store("magicka-save-editor", Some("config"), &config)?;
    dbg!(&config);
    eframe::run_native(
        "Magicka Save Editor",
        eframe::NativeOptions::default(),
        Box::new(|cc| Box::new(magicka_save_editor::app::App::new(cc))),
    );
    Ok(())
}
