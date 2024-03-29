/*!

 */
use std::fs::File;
use std::io::{BufReader, Cursor, Error, ErrorKind, Read, Seek, SeekFrom};
use std::path::PathBuf;
use std::result;
use byteordered::byteorder::{BigEndian, ReadBytesExt};

pub type Result<T> = result::Result<T, Error>;
/// The byte type in the heap dump file
pub type Byte = u8;
/// The short type in the heap dump file
pub type Short = u16;
/// The int type in the heap dump file
pub type Int = u32;
/// The long type in the heap dump file
pub type Long = u64;
/// The float type in the heap dump file
pub type Float = f32;
/// The double type in the heap dump file
pub type Double = f64;
/// The char type in the heap dump file
pub type Char = char;
/// The boolean type in the heap dump file
pub type Boolean = bool;

/// # Channel used to read snapshot files
pub struct Channel {
    /// a snapshot file
    file: File,
}

impl Channel {
    /// Open a file to build a channel
    /// # Examples
    /// ```rust
    /// use io::channel::{Channel, Result};
    ///
    /// let file_path = ...
    /// let channel = Channel::open(&file_path);
    /// ```
    pub fn open(file_path: &PathBuf) -> Result<Self> {
        let file = File::open(file_path.clone())?;
        Ok(Self { file })
    }

    fn read(&mut self, len: usize) -> Result<Vec<u8>> {
        let mut buf: Vec<u8> = vec![Default::default(); len];
        let count: usize = self.file.read(&mut buf)?;
        if count == 0 || buf.is_empty() {
            return Err(Error::from(ErrorKind::UnexpectedEof));
        }
        Ok(buf)
    }

    pub fn skip(&mut self, len: i64) {
        let _ = self.file.seek(SeekFrom::Current(len));
    }

    pub fn read_byte(&mut self) -> Result<Byte> {
        self.file.read_u8()
    }

    pub fn read_bool(&mut self) -> Result<Boolean> {
        let val = self.file.read_u8()?;
        Ok(val != 0)
    }

    pub fn read_short(&mut self) -> Result<Short> {
        self.file.read_u16::<BigEndian>()
    }

    pub fn read_int(&mut self) -> Result<Int> {
        self.file.read_u32::<BigEndian>()
    }

    pub fn read_long(&mut self) -> Result<Long> {
        self.file.read_u64::<BigEndian>()
    }

    pub fn read_float(&mut self) -> Result<Float> {
        self.file.read_f32::<BigEndian>()
    }

    pub fn read_double(&mut self) -> Result<Double> {
        self.file.read_f64::<BigEndian>()
    }

    pub fn read_char(&mut self) -> Result<Char> {
        let str = self.read_str(1)?;
        let byte = str.chars().next().unwrap();
        Ok(byte)
    }

    pub fn read_str(&mut self, len: usize) -> Result<String> {
        let buf: Vec<u8> = self.read(len)?;
        let result = String::from_utf8_lossy(&buf);
        Ok(result.to_string())
    }

    /// The current position where the file is being read
    pub fn position(&mut self) -> Result<u64> {
        self.file.seek(SeekFrom::Current(0))
    }

}