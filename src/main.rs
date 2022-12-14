use magicka_save_editor::save::Save;

fn main() -> eyre::Result<()> {
    // let path = "campaign.sav";
    let path = "campaign2.sav";
    let reader = std::io::BufReader::new(std::fs::File::open(path)?);
    let mut save = Save::new(reader);
    let camp = save.load_campaign()?;
    // dbg!(&camp);
    println!("{:X?}", camp);
    Ok(())
}
