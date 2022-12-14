use std::collections::HashMap;
use std::io::{Read, Write};
use std::num;

use byteorder::LittleEndian;
use byteorder::ReadBytesExt;

use super::Error;
use super::{read_len_string, Save};

const TIPS_NAMES: [&str; 11] = [
    "#tu_text_hint_equipment_key",
    "#tu_text_hint_equipment_pad",
    "#tu_text_hint_wet_lightning",
    "#tu_text_hint_wet",
    "#tu_text_hint_cold",
    "#tu_text_hint_poison",
    "#tip09",
    "#tip10",
    "#tip15",
    "#tip17",
    "#tip18",
];

/// internal representation of a save slot
#[derive(Debug)]
pub struct SaveSlot {
    buffer: Box<[u8]>,
    level: u8,
    max_allowed_level: u8,
    looped: bool,
    total_playtime: i32,
    current_playtime: i32,
    players: HashMap<String, PlayerSaveData>,
    unlocked_magicks: u64,
    shown_tips: Vec<Tip>,
    checkpoint: Vec<u8>, // still dont really know what this is
}

#[derive(Debug)]
pub struct PlayerSaveData {
    staff: String,
    weapon: String,
}

#[derive(Debug)]
pub struct Tip {
    name: String,
    timestamp: f64,
    count: i32,
}

impl SaveSlot {
    pub fn read<R: Read>(mut reader: &mut R, version_num: u64) -> Result<Self, Error> {
        if version_num >= 0x1000400010000 {
            Self::read_v1410(&mut reader)
        } else {
            Self::read_v1000(&mut reader)
        }
    }

    fn read_v1410<R: Read>(mut reader: &mut R) -> Result<Self, Error> {
        let level = reader.read_u8()?;
        let max_allowed_level = reader.read_u8()?;
        let looped = if reader.read_u8()? == 0 { false } else { true };
        let total_playtime = reader.read_i32::<LittleEndian>()?;
        let current_playtime = reader.read_i32::<LittleEndian>()?;

        // read players
        let num_players = reader.read_i32::<LittleEndian>()?;
        let mut players: HashMap<String, PlayerSaveData> = HashMap::new();
        for _ in 0..num_players {
            let name = read_len_string(&mut reader)?;
            let player_data = PlayerSaveData::read(&mut reader)?;
            players.insert(name, player_data);
        }

        let unlocked_magicks = reader.read_u64::<LittleEndian>()?;

        // read tips
        let num_tips = reader.read_i32::<LittleEndian>()?;
        let mut shown_tips = Vec::with_capacity(11);
        for _ in 0..num_tips {
            let tip = Tip::read(&mut reader)?;
            shown_tips.push(tip);
        }

        // read checkpoint
        let mut buffer = vec![0u8; 1024].into_boxed_slice();
        let mut num_checkpoints = reader.read_i32::<LittleEndian>()? as usize;
        let mut checkpoint = Vec::with_capacity(num_checkpoints as usize);
        if num_checkpoints > 0 {
            while num_checkpoints > 0 {
                let count = reader
                    .take(std::cmp::min(buffer.len() as u64, num_checkpoints as u64))
                    .read(&mut buffer)?;
                checkpoint.write_all(&buffer)?;
                num_checkpoints -= count;
            }
            // TODO: maybe need to set checkpoint.position to 0 here
        }

        // good now maybe?
        Ok(SaveSlot {
            buffer,
            level,
            max_allowed_level,
            looped,
            total_playtime,
            current_playtime,
            players,
            unlocked_magicks,
            shown_tips,
            checkpoint,
        })
    }

    fn read_v1000<R: Read>(mut reader: &mut R) -> Result<Self, Error> {
        let level = reader.read_u8()?;
        let max_allowed_level = level;
        let looped = if reader.read_u8()? == 0 { false } else { true };
        let total_playtime = reader.read_i32::<LittleEndian>()?;
        let current_playtime = reader.read_i32::<LittleEndian>()?;

        // read players
        let num_players = reader.read_i32::<LittleEndian>()?;
        let mut players: HashMap<String, PlayerSaveData> = HashMap::new();
        for _ in 0..num_players {
            let name = read_len_string(&mut reader)?;
            let player_data = PlayerSaveData::read(&mut reader)?;
            players.insert(name, player_data);
        }

        let unlocked_magicks = reader.read_u64::<LittleEndian>()?;

        // read tips
        let num_tips = reader.read_i32::<LittleEndian>()?;
        let mut shown_tips = Vec::with_capacity(11);
        for _ in 0..num_tips {
            let tip = Tip::read(&mut reader)?;
            shown_tips.push(tip);
        }

        Ok(SaveSlot {
            level,
            max_allowed_level,
            looped,
            total_playtime,
            current_playtime,
            players,
            unlocked_magicks,
            shown_tips,
            ..Default::default()
        })
    }
}

impl PlayerSaveData {
    pub fn read<R: Read>(mut reader: &mut R) -> Result<Self, Error> {
        let staff = read_len_string(&mut reader)?;
        let weapon = read_len_string(&mut reader)?;
        Ok(PlayerSaveData { staff, weapon })
    }
}

impl Tip {
    pub fn read<R: Read>(mut reader: &mut R) -> Result<Self, Error> {
        let name = read_len_string(&mut reader)?;
        let timestamp = f64::NEG_INFINITY;
        let count = reader.read_i32::<LittleEndian>()?;
        Ok(Tip {
            name,
            timestamp,
            count,
        })
    }
}

impl Default for SaveSlot {
    fn default() -> Self {
        Self {
            buffer: vec![0u8; 1024].into_boxed_slice(),
            level: 0,
            max_allowed_level: 0,
            looped: false,
            total_playtime: 0,
            current_playtime: 0,
            players: HashMap::new(),
            unlocked_magicks: 0,
            shown_tips: TIPS_NAMES
                .iter()
                .map(|&x| Tip {
                    name: String::from(x),
                    timestamp: f64::NEG_INFINITY,
                    count: 0,
                })
                .collect(),
            checkpoint: Vec::new(),
        }
    }
}
