use std::collections::HashMap;
use std::io::{Read, Write};
use std::num;

use byteorder::{LittleEndian, WriteBytesExt};
use byteorder::ReadBytesExt;

use super::{Error, write_boolean, write_len_string};
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
    // buffer: Box<[u8]>,
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
        let checkpoint_len = reader.read_i32::<LittleEndian>()? as usize;
        let mut checkpoint = vec![0u8; checkpoint_len];
        if checkpoint_len > 0 {
            reader.take(checkpoint_len as u64).read_exact(&mut checkpoint)?;
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

    pub fn write<W: Write>(&self, mut writer: &mut W) -> Result<(), Error> {
        writer.write_u8(self.level)?;
        writer.write_u8(self.max_allowed_level)?;
        write_boolean(&mut writer, self.looped)?;
        writer.write_i32::<LittleEndian>(self.total_playtime)?;
        writer.write_i32::<LittleEndian>(self.current_playtime)?;

        // write players
        writer.write_i32::<LittleEndian>(self.players.len() as i32)?;
        for (player_name, player_data) in &self.players {
            write_len_string(&mut writer, player_name)?;
            player_data.write(&mut writer)?;
        }

        writer.write_u64::<LittleEndian>(self.unlocked_magicks)?;

        // write shown tips
        writer.write_i32::<LittleEndian>(self.shown_tips.len() as i32)?;
        for tip in &self.shown_tips {
            tip.write(&mut writer)?;
        }

        // write checkpoint
        let checkpoint_len = self.checkpoint.len() as i32;
        writer.write_i32::<LittleEndian>(checkpoint_len)?;
        if checkpoint_len > 0 {
            writer.write_all(&self.checkpoint)?;
        }

        Ok(())
    }

    pub fn print(&self) {
        println!("Current Chapter  : {}", self.level);
        println!("Maximum Chapter  : {}", self.max_allowed_level);
        println!("Looped (NG+)     : {}", self.looped);
        println!("Current Playtime : {}", self.current_playtime);
        println!("Total Playtime   : {}", self.total_playtime);
        println!("Players ----------------");
        for player in self.players.iter() {
            println!("  Name   : {}", player.0);
            println!("  Staff  : {}", player.1.staff);
            println!("  Weapon : {}", player.1.weapon);
        }
    }
}

impl PlayerSaveData {
    pub fn read<R: Read>(mut reader: &mut R) -> Result<Self, Error> {
        let staff = read_len_string(&mut reader)?;
        let weapon = read_len_string(&mut reader)?;
        Ok(PlayerSaveData { staff, weapon })
    }

    pub fn write<W: Write>(&self, mut writer: &mut W) -> Result<(), Error> {
        write_len_string(&mut writer, &self.staff)?;
        write_len_string(&mut writer, &self.weapon)?;
        Ok(())
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

    pub fn write<W: Write>(&self, mut writer: &mut W) -> Result<(), Error> {
        write_len_string(&mut writer, &self.name)?;
        writer.write_i32::<LittleEndian>(self.count)?;
        Ok(())
    }
}

impl Default for SaveSlot {
    fn default() -> Self {
        Self {
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
