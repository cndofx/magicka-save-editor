use std::io::Read;

use magicka_save_editor::save::Save;

fn main() -> eyre::Result<()> {
    let path = "campaign.sav";
    // let save = save::Save::open(path);
    // println!("{:X?}", save);
    let testdata = vec![0u8, 0, 0, 0];
    let reader = std::io::BufReader::new(std::fs::File::open(path)?);
    let mut save = Save::new(reader);
    dbg!(&save);
    let camp = save.load_campaign()?;
    // let save2 = Save::new(std::io::BufReader::new(&testdata[..]));
    // dbg!(&save2);

    Ok(())
}
