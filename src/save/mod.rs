use std::alloc::System;
use std::fs::File;
use std::io::{BufRead, BufReader, Read, Seek};
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use byteorder::ReadBytesExt;
use tap::{Conv, Tap};

use self::internal::SaveSlot;

mod internal;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("system time error: {0}")]
    SystemTimeError(#[from] std::time::SystemTimeError),
    #[error("int parsing error: {0}")]
    IntParsingError(#[from] std::num::ParseIntError),
    #[error("utf8 string conversion error: {0}")]
    UTF8Error(#[from] std::str::Utf8Error),
    #[error("io error: {0}")]
    IOError(#[from] std::io::Error),
    #[error("placeholder error wrapper over {0}")]
    Placeholder(#[from] eyre::Report),
}

/// main save structure that handles deserializing a save file
#[derive(Debug)]
pub struct Save<R> {
    reader: BufReader<R>,
    done: bool,
}

/// structure containing deserialized save data
#[derive(Debug)]
pub struct SaveInfo {
    product_version: String,
    save_slots: Vec<SaveSlot>,
}

impl<R: Read + Seek> Save<R> {
    pub fn new(reader: BufReader<R>) -> Self {
        Save {
            reader,
            done: false,
        }
    }

    pub fn load_campaign(&mut self) -> Result<SaveInfo, Error> {
        // TODO: load_campaign() should either consume self or set self.done to prevent save from being read twice
        let mut version = String::new();
        let mut version_num = 0;

        // read version information
        if self.reader.read_u8()? == 0xFF {
            version = read_len_string(&mut self.reader)?;
            let nums: Vec<u64> = version
                .split(".")
                .map(|s| s.parse::<u64>())
                .collect::<Result<Vec<_>, _>>()?;
            version_num = nums[0] << 48 | nums[1] << 32 | nums[2] << 16 | nums[3];
        } else {
            self.reader.seek_relative(-1)?;
        }

        // read all 3 save slots
        let mut save_slots: Vec<SaveSlot> = Vec::new();
        for _ in 0..3 {
            if read_boolean(&mut self.reader)? {
                save_slots.push(SaveSlot::read(&mut self.reader, version_num)?);
            }
        }

        Ok(SaveInfo {
            product_version: version,
            save_slots,
        })
    }
}

impl SaveInfo {
    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<(), Error>
    {
        let path = PathBuf::from(path.as_ref());

        // backup save if it already exists to prevent data loss
        if path.exists() {
            backup_file(&path)?;
        }



        todo!()
    }

    pub fn print(&self) {
        for (i, slot) in self.save_slots.iter().enumerate() {
            println!("Save Slot {} ============", i + 1);
            slot.print();
            println!("");
        }
    }
}

/// read string prefixed with a byte specifying the length
fn read_len_string<R: Read>(reader: &mut R) -> Result<String, Error> {
    let len = reader.read_u8()?;
    let mut string = vec![0u8; len as usize];
    reader.read_exact(&mut string)?;
    Ok(std::str::from_utf8(&string)?.to_owned())
}

fn read_boolean<R: Read>(reader: &mut R) -> Result<bool, Error> {
    let byte = reader.read_u8()?;
    Ok(if byte == 0 { false } else { true })
}

fn backup_file(path: &Path) -> Result<(), Error>
{
    let path = PathBuf::from(&path);
    let time = SystemTime::now()
        .duration_since(UNIX_EPOCH)?
        .as_millis();
    let backup_path = path
        .clone()
        .into_os_string()
        .tap_mut(|s| s.push(format!(".bak-{time}")))
        .conv::<PathBuf>();
    std::fs::copy(path, backup_path)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_read_len_string() {
        let bytes: [u8; 12] = [
            0x0B, 0x61, 0x62, 0x63, 0x64, 0x20, 0x78, 0x79, 0x7A, 0x20, 0x21, 0x3F,
        ];
        let mut reader = BufReader::new(bytes.as_slice());
        let string = read_len_string(&mut reader).unwrap();
        assert_eq!(string, "abcd xyz !?");
    }
}
