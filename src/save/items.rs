use std::path::{Path, PathBuf};

pub fn get_items<P: AsRef<Path>>(game_path: P) -> Vec<String> {
    let mut game_path = PathBuf::from(game_path.as_ref());
    game_path.push("Content/Data/items/Wizard/");
    let mut names = game_path
        .read_dir()
        .unwrap()
        .filter_map(|entry| entry.ok())
        .filter_map(|entry| entry.file_name().into_string().ok())
        .map(|entry| entry.trim_end_matches(".xnb").to_string())
        .collect::<Vec<_>>();
    names.sort();
    names
}

pub fn get_staves<P: AsRef<Path>>(game_path: P) -> Vec<String> {
    let items = get_items(game_path);
    items.into_iter().filter(|item| item.starts_with("staff_")).collect()
}

pub fn get_weapons<P: AsRef<Path>>(game_path: P) -> Vec<String> {
    let items = get_items(game_path);
    items.into_iter().filter(|item| item.starts_with("weapon_")).collect()
}