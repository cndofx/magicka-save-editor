use std::path::{Path, PathBuf};

pub fn get_staves<P: AsRef<Path>>(game_path: P) -> Vec<String> {
    let mut game_path = PathBuf::from(game_path.as_ref());
    game_path.push("Content/Data/items/Wizard/");
    dbg!(game_path);
    todo!()
}