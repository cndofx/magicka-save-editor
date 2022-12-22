use magicka_save_editor::{Config, CONFY_APP_NAME, CONFY_CONFIG_NAME};

fn main() -> eyre::Result<()> {
    let config: Config = confy::load(CONFY_APP_NAME, CONFY_CONFIG_NAME)?;
    confy::store(CONFY_APP_NAME, CONFY_CONFIG_NAME, &config)?;
    dbg!(&config);
    eframe::run_native(
        "Magicka Save Editor",
        eframe::NativeOptions::default(),
        Box::new(|cc| Box::new(magicka_save_editor::app::App::new(cc, config))),
    );
    Ok(())
}
