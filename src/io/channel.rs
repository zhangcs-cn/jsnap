use std::fs::File;
use std::io::{BufReader, Cursor, Error, ErrorKind, Read, Seek, SeekFrom};
use std::path::PathBuf;
use std::result;
use byteordered::byteorder::{BigEndian, ReadBytesExt};

pub type Result<T> = result::Result<T, Error>;

pub struct Channel {
    reader: BufReader<File>,
}

impl Channel {

    pub fn open(file_path: PathBuf) -> Result<Self> {
        let file = File::open(file_path.clone())?;
        let mut reader = BufReader::new(file);
        Ok(Self { reader })
    }

    fn read(&mut self, len: usize) -> Result<Vec<u8>> {
        let mut buf: Vec<u8> = vec![Default::default(); len];
        let count: usize = self.reader.read(&mut buf)?;
        if count == 0 || buf.is_empty() {
            return Err(Error::from(ErrorKind::UnexpectedEof));
        }
        Ok(buf)
    }

    pub fn skip(&mut self, len: i64) {
        let _ = self.reader.seek(SeekFrom::Current(len));
    }

    pub fn read_u8(&mut self) -> Result<u8> {
        let buf: Vec<u8> = self.read(1)?;
        let mut cursor: Cursor<Vec<u8>> = Cursor::new(buf);
        let number: u8 = cursor.read_u8()?;
        Ok(number)
    }

    pub fn read_u16(&mut self) -> Result<u16> {
        let buf: Vec<u8> = self.read(2)?;
        let mut cursor: Cursor<Vec<u8>> = Cursor::new(buf);
        let number: u16 = cursor.read_u16::<BigEndian>()?;
        Ok(number)
    }

    pub fn read_u32(&mut self) -> Result<u32> {
        let buf: Vec<u8> = self.read(4)?;
        let mut cursor: Cursor<Vec<u8>> = Cursor::new(buf);
        let number: u32 = cursor.read_u32::<BigEndian>()?;
        Ok(number)
    }

    pub fn read_u64(&mut self) -> Result<u64> {
        let buf: Vec<u8> = self.read(8)?;
        let mut cursor: Cursor<Vec<u8>> = Cursor::new(buf);
        let number: u64 = cursor.read_u64::<BigEndian>()?;
        Ok(number)
    }

    pub fn read_str(&mut self, len: usize) -> Result<String> {
        let buf: Vec<u8> = self.read(len)?;
        let result = String::from_utf8_lossy(&buf);
        Ok(result.to_string())
    }

    pub fn read_char(&mut self) -> Result<char> {
        let str = self.read_str(1)?;
        let byte = str.chars().next().unwrap();
        Ok(byte)
    }

    pub fn position(&mut self) -> Result<u64> {
        self.reader.seek(SeekFrom::Current(0))
    }

}
