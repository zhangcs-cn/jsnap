use std::fs::File;
use std::io::{Error, ErrorKind, Read, Cursor, SeekFrom, Seek};
use std::path::Path;
use std::str::Utf8Error;
use std::mem::transmute;
use byteordered::byteorder::{ReadBytesExt, BigEndian};

pub(crate) struct Channel {
    file: File,
    id_size: u8,
}

impl Channel {
    pub(crate) fn new(path: &Path) -> Result<Channel, Error> {
        let mut file = File::open(path)?;
        let channel = Channel { file, id_size: 8 };
        Ok(channel)
    }

    pub(crate) fn read_str(&mut self, len: usize) -> Result<String, Error> {
        let mut buf = self.read(len)?;
        let result = std::str::from_utf8(&buf).unwrap();
        Ok(result.to_string())
    }

    pub(crate) fn read_u64(&mut self) -> Result<u64, Error> {
        let buf = self.read(8)?;
        let mut cursor = Cursor::new(buf);
        let number: u64 = cursor.read_u64::<BigEndian>()?;
        Ok(number)
    }

    pub(crate) fn read_u32(&mut self) -> Result<u32, Error> {
        let buf = self.read(4)?;
        let mut cursor = Cursor::new(buf);
        let number: u32 = cursor.read_u32::<BigEndian>()?;
        Ok(number)
    }

    pub(crate) fn read_u8(&mut self) -> Result<u8, Error> {
        let buf = self.read(1)?;
        let mut cursor = Cursor::new(buf);
        let number: u8 = cursor.read_u8()?;
        Ok(number)
    }

    pub(crate) fn read_u16(&mut self) -> Result<u16, Error> {
        let buf = self.read(2)?;
        let mut cursor = Cursor::new(buf);
        let number: u16 = cursor.read_u16::<BigEndian>()?;
        Ok(number)
    }

    pub(crate) fn skip(&mut self, len: i64) {
        let _ = self.file.seek(SeekFrom::Current(len));
    }

    pub(crate) fn reads(&mut self, buf: &mut [u8]) -> usize {
        return self.file.read(buf).unwrap();
    }

    pub(crate) fn read(&mut self, len: usize) -> Result<Vec<u8>, Error> {
        let mut buf = vec![Default::default(); len];
        let count = self.file.read(&mut buf)?;
        if count <= 0 {
            Err(Error::new(ErrorKind::Other, "EOF"))
        } else {
            Ok(buf)
        }
    }

    pub(crate) fn read_id(&mut self) -> Result<u64, Error> {
        let mut id: u64;
        if self.id_size == 4 {
            id = self.read_u32()? as u64;
        } else {
            id = self.read_u64()?;
        }
        Ok(id)
    }

    pub(crate) fn set_id_size(&mut self, id_size: u8) {
        self.id_size = id_size
    }

}