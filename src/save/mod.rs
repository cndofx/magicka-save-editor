use std::fs::File;
use std::io::{BufReader, Read, BufRead, Seek};
use std::path::PathBuf;

use byteorder::ReadBytesExt;

use self::internal::SaveSlot;

mod internal;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("int parsing error: {0}")]
    IntParsingError(#[from] std::num::ParseIntError),
    #[error("utf8 string conversion error: {0}")]
    UTF8Error(#[from] std::str::Utf8Error),
    #[error("io error: {0}")]
    IOError(#[from] std::io::Error),
    #[error("placeholder error wrapper over {0}")]
    Placeholder(#[from] eyre::Report)
}

/// main save structure that handles all operations
#[derive(Debug)]
pub struct Save<R> {
    reader: BufReader<R>,
    done: bool,
}

impl<R: Read + Seek> Save<R> {
    pub fn new(reader: BufReader<R>) -> Self {
        Save { reader, done: false }
    }

    pub fn load_campaign(&mut self) -> Result<SaveInfo, Error> {
        let mut version = String::new();
        let mut version_num = 0;

        // read version information
        if self.reader.read_u8()? == 0xFF {
            version = read_len_string(&mut self.reader)?;
            let nums: Vec<u64> = version.split(".").map(|s| s.parse::<u64>()).collect::<Result<Vec<_>, _>>()?;
            version_num = nums[0] << 48 | nums[1] << 32 | nums[2] << 16 | nums[3];
        } else {
            self.reader.seek_relative(-1)?;
        }
        // println!("current version str: {version}");
        // println!("current version num: {:X}", version_num);

        // read all 3 save slots
        let mut save_slots: Vec<SaveSlot> = Vec::new();
        for _ in 0..3 {
            if read_boolean(&mut self.reader)? {
                save_slots.push(SaveSlot::read(&mut self.reader, version_num)?);
            }
        }

        // println!("{:X?}", save_slots);
        
        Ok(SaveInfo {
            product_version: version,
            save_slots,
        })
    }
}

#[derive(Debug)]
pub struct SaveInfo {
    product_version: String,
    save_slots: Vec<SaveSlot>,
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_read_len_string() {
        let bytes: [u8; 12] = [0x0B, 0x61, 0x62, 0x63, 0x64, 0x20, 0x78, 0x79, 0x7A, 0x20, 0x21, 0x3F];
        let mut reader = BufReader::new(bytes.as_slice());
        let string = read_len_string(&mut reader).unwrap();
        assert_eq!(string, "abcd xyz !?");
    }
}