use magicka_save_editor::save::Save;

fn main() -> eyre::Result<()> {
    // let path = "campaign.sav";
    // let out_path = "campaign-modified.sav";
    let path = "campaign2.sav";
    let out_path = "campaign2-modified.sav";
    let reader = std::io::BufReader::new(std::fs::File::open(path)?);
    let mut save = Save::new(reader);
    let camp = save.load_campaign()?;
    camp.print();
    camp.save_to_file(out_path)?;
    Ok(())
}
