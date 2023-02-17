use std::fs::File;
use std::io::{Error, ErrorKind, Read};
use std::path::Path;
use std::str::Utf8Error;

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
        let mut buf = vec![Default::default(); len];
        let count = self.file.read(&mut buf)?;
        if count <= 0 {
            return Err(Error::new(ErrorKind::Other, ""));
        }
        let result = std::str::from_utf8(&buf).unwrap();
        Ok(result.to_string())
    }

    pub(crate) fn read_() {
        let mut buf = [0u8; 8];
    }

    fn read(&mut self, len: usize) -> Vec<u8> {
        let mut buf = vec![Default::default(); len];
        let count = self.file.read(&mut buf).unwrap();
        if count <= 0 {

        }
        buf
    }

}