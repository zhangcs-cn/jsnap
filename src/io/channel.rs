use std::fs::File;
use std::io::{Error, ErrorKind, Read, Cursor, SeekFrom, Seek};
use std::path::Path;
use std::str::Utf8Error;
use std::mem::transmute;
use byteordered::byteorder::{ReadBytesExt, BigEndian};

pub(crate) struct HprofChannel {
    file: File,
}

impl HprofChannel {
    pub(crate) fn new(path: &Path) -> Result<HprofChannel, Error> {
        let mut file = File::open(path)?;
        let channel = HprofChannel { file };
        Ok(channel)
    }

    pub(crate) fn read_str(&mut self, len: usize) -> Result<String, Error> {
        let mut buf = self.read(len)?;
        let result = std::str::from_utf8(&buf).unwrap();
        Ok(result.to_string())
    }

    pub(crate) fn read_u64(&mut self) -> Result<u64, Error> {
        let mut buf = self.read(8)?;
        let mut cursor = Cursor::new(buf);
        let mut number: u64 = cursor.read_u64::<BigEndian>()?;
        Ok(number)
    }

    pub(crate) fn read_u32(&mut self) -> Result<u32, Error> {
        let mut buf = self.read(4)?;
        let mut cursor = Cursor::new(buf);
        let mut number: u32 = cursor.read_u32::<BigEndian>()?;
        Ok(number)
    }

    pub(crate) fn read_u8(&mut self) -> Result<u8, Error> {
        let mut buf = self.read(1)?;
        let mut cursor = Cursor::new(buf);
        let mut number: u8 = cursor.read_u8()?;
        Ok(number)
    }

    pub(crate) fn read_u2(&mut self) {}

    pub(crate) fn skip(&mut self, len: i64) -> Result<u64, Error> {
        self.file.seek(SeekFrom::Current(len))
    }

    fn read(&mut self, len: usize) -> Result<Vec<u8>, Error> {
        let mut buf = vec![Default::default(); len];
        let count = self.file.read(&mut buf)?;
        if count <= 0 {
            Err(Error::new(ErrorKind::Other, "EOF"))
        } else {
            Ok(buf)
        }
    }
}