use byteordered::byteorder::{BigEndian, ReadBytesExt};
use std::fs::File;
use std::io::{Cursor, Error, Read, Seek, SeekFrom};
use std::path::Path;
use std::result;
use std::str::Utf8Error;
use super::error::{EndOfFile};

pub type Result<T> = result::Result<T, Error>;
pub type Byte = u8;
pub type Short = u16;
pub type Int = u32;
pub type Long = u64;

pub struct Channel {
    file: File,
    size: u64,
}

impl Channel {
    fn reads(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.file.read(buf)
    }

    fn read(&mut self, len: usize) -> Result<Vec<u8>> {
        let mut buf: Vec<u8> = vec![Default::default(); len];
        let count: usize = self.file.read(&mut buf)?;
        if count == 0 || buf.is_empty() {
            return Err(Error::from(EndOfFile));
        }
        Ok(buf)
    }

    pub fn skip(&mut self, len: i64) {
        let _ = self.file.seek(SeekFrom::Current(len));
    }

    pub fn read_byte(&mut self) -> Result<Byte> {
        let buf: Vec<u8> = self.read(1)?;
        let mut cursor: Cursor<Vec<u8>> = Cursor::new(buf);
        let number: u8 = cursor.read_u8()?;
        Ok(number)
    }

    pub fn read_short(&mut self) -> Result<Short> {
        let buf: Vec<u8> = self.read(2)?;
        let mut cursor: Cursor<Vec<u8>> = Cursor::new(buf);
        let number: u16 = cursor.read_u16::<BigEndian>()?;
        Ok(number)
    }

    pub fn read_int(&mut self) -> Result<Int> {
        let buf: Vec<u8> = self.read(4)?;
        let mut cursor: Cursor<Vec<u8>> = Cursor::new(buf);
        let number: u32 = cursor.read_u32::<BigEndian>()?;
        Ok(number)
    }

    pub fn read_long(&mut self) -> Result<Long> {
        let buf: Vec<u8> = self.read(8)?;
        let mut cursor: Cursor<Vec<u8>> = Cursor::new(buf);
        let number: u64 = cursor.read_u64::<BigEndian>()?;
        Ok(number)
    }

    pub fn read_str(&mut self, len: usize) -> Result<String> {
        let mut buf: Vec<u8> = self.read(len)?;
        let result = String::from_utf8_lossy(&buf);
        Ok(result.to_string())
    }

    pub fn read_char(&mut self) -> Result<char> {
        let str = self.read_str(1)?;
        let byte = str.chars().next().unwrap();
        Ok(byte)
    }

    pub fn new(file_path: &Path) -> Result<Channel> {
        let mut file = File::open(file_path)?;
        let metadata = std::fs::metadata(file_path)?;
        let len = metadata.len();
        let channel = Channel { file, size: len };
        Ok(channel)
    }
}