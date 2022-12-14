use std::collections::HashMap;
use std::io::Read;
use std::num;

use byteorder::LittleEndian;
use byteorder::ReadBytesExt;

use super::read_len_string;
use super::Error;

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
    // unsure what these are for now
    // private MemoryStream mCheckPoint;
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

        let num_players = reader.read_i32::<LittleEndian>()?;
        let mut players: HashMap<String, PlayerSaveData> = HashMap::new();
        for _ in 0..num_players {
            let name = read_len_string(&mut reader)?;
            let player_data = PlayerSaveData::read(&mut reader)?;
            players.insert(name, player_data);
        }

        let unlocked_magicks = reader.read_u64::<LittleEndian>()?;

        let num_tips = reader.read_i32::<LittleEndian>()?;
        let mut shown_tips = Vec::with_capacity(11);
        for _ in 0..num_tips {
            let tip = Tip::read(&mut reader)?;
            shown_tips.push(tip);
        }

        let num_checkpoints = reader.read_i32::<LittleEndian>()?;
        if num_checkpoints > 0 {

        }

        todo!()
    }

    fn read_v1000<R: Read>(reader: &mut R) -> Result<Self, Error> {
        let level = reader.read_u8()?;
        let max_allowed_level = level;
        let looped = if reader.read_u8()? == 0 { false } else { true };
        // TODO: make sure endianness is correct
        let total_playtime = reader.read_i32::<LittleEndian>()?;
        let current_playtime = reader.read_i32::<LittleEndian>()?;
        // let num_players =
        todo!()
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
