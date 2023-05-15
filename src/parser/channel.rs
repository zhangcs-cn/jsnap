use std::fs::File;
use std::io::{BufReader, Cursor, Error, ErrorKind, Read, Seek, SeekFrom};
use std::path::PathBuf;
use std::result;
use byteordered::byteorder::{BigEndian, ReadBytesExt};

pub type Result<T> = result::Result<T, Error>;
pub type Byte = u8;
pub type Short = u16;
pub type Int = u32;
pub type Long = u64;

/// 文件流
pub struct Channel {
    reader: BufReader<File>,
    file_len: usize,
}

impl Channel {
    pub fn new(file_path: PathBuf) -> Result<Channel> {
        let file_path_str = file_path.to_str().unwrap();
        let file = File::open(file_path.clone());
        if file.is_err() {
            return Err(Error::new(ErrorKind::NotFound, format!("文件打开失败: {}", file_path_str)));
        }
        let file = file.unwrap();
        let meta = file.metadata();
        if meta.is_err() {
            return Err(Error::new(ErrorKind::BrokenPipe, format!("读取文件失败: {}", file_path_str)));
        }
        let file_len = meta.unwrap().len() as usize;
        let mut reader = BufReader::new(file);
        Ok(Channel { reader, file_len })
    }

    pub fn get_file_len(&self) -> usize {
        self.file_len
    }

    fn reads(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.reader.read(buf)
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
        let buf: Vec<u8> = self.read(len)?;
        let result = String::from_utf8_lossy(&buf);
        Ok(result.to_string())
    }

    pub fn read_char(&mut self) -> Result<char> {
        let str = self.read_str(1)?;
        let byte = str.chars().next().unwrap();
        Ok(byte)
    }
}