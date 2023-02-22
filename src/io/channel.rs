use std::fs::File;
use std::io::{Error, ErrorKind, Read, Cursor, SeekFrom, Seek};
use std::path::Path;
use byteordered::byteorder::{ReadBytesExt, BigEndian};

pub trait Channel {
    fn reads(&mut self, buf: &mut [u8]) -> usize;
    fn read(&mut self, len: usize) -> Result<Vec<u8>, Error>;
    fn skip(&mut self, len: i64);
    fn read_u8(&mut self) -> Result<u8, Error> {
        let buf: Vec<u8> = self.read(1)?;
        let mut cursor: Cursor<Vec<u8>> = Cursor::new(buf);
        let number: u8 = cursor.read_u8()?;
        Ok(number)
    }
    fn read_u16(&mut self) -> Result<u16, Error> {
        let buf: Vec<u8> = self.read(2)?;
        let mut cursor: Cursor<Vec<u8>> = Cursor::new(buf);
        let number: u16 = cursor.read_u16::<BigEndian>()?;
        Ok(number)
    }
    fn read_u32(&mut self) -> Result<u32, Error> {
        let buf: Vec<u8> = self.read(4)?;
        let mut cursor: Cursor<Vec<u8>> = Cursor::new(buf);
        let number: u32 = cursor.read_u32::<BigEndian>()?;
        Ok(number)
    }
    fn read_u64(&mut self) -> Result<u64, Error> {
        let buf: Vec<u8> = self.read(8)?;
        let mut cursor: Cursor<Vec<u8>> = Cursor::new(buf);
        let number: u64 = cursor.read_u64::<BigEndian>()?;
        Ok(number)
    }
    fn read_str(&mut self, len: usize) -> Result<String, Error> {
        let mut buf: Vec<u8> = self.read(len)?;
        let result: &str = std::str::from_utf8(&buf).unwrap();
        Ok(result.to_string())
    }
}

pub struct HprofChannel {
    file: File,
    size: u64,
}

impl Channel for HprofChannel {
    fn reads(&mut self, buf: &mut [u8]) -> usize {
        return self.file.read(buf).unwrap();
    }

    fn read(&mut self, len: usize) -> Result<Vec<u8>, Error> {
        let mut buf: Vec<u8> = vec![Default::default(); len];
        let count: usize = self.file.read(&mut buf)?;
        if count <= 0 {
            Err(Error::new(ErrorKind::Other, "EOF"))
        } else {
            Ok(buf)
        }
    }

    fn skip(&mut self, len: i64) {
        let _ = self.file.seek(SeekFrom::Current(len));
    }
}

impl HprofChannel {
    pub fn new(file_path: &Path) -> Result<HprofChannel, Error> {
        let mut file = File::open(file_path);
        return if file.is_ok() {
            let file = file.unwrap();
            let metadata = std::fs::metadata(file_path).unwrap();
            let len = metadata.len();
            let channel = HprofChannel { file, size: len };
            Ok(channel)
        } else {
            Err(Error::new(ErrorKind::NotFound, file.err().unwrap()))
        };
    }
}