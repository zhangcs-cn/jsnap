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
        let mut buf = self.read(len)?;
        let result = std::str::from_utf8(&buf).unwrap();
        Ok(result.to_string())
    }

    pub(crate) fn read_u64(&mut self) -> Result<u64, Error> {
        let mut buf = self.read(8)?;
        let ptr: *const u8 = buf.as_ptr();
        let ptr: *const u64 = ptr as *const u64;
        let r = unsafe { *ptr };
        Ok(r)
    }

    pub(crate) fn read_u32(&mut self) -> Result<u32, Error> {
        let mut buf = self.read(4)?;
        let ptr: *const u8 = buf.as_ptr();
        let ptr: *const u32 = ptr as *const u32;
        let r = unsafe { *ptr };
        Ok(r)
    }

    pub(crate) fn read_u8(&mut self) -> Result<u8, Error> {
        let mut buf = self.read(1)?;
        let ptr: *const u8 = buf.as_ptr();
        let r = unsafe { *ptr };
        Ok(r)
    }

    pub(crate) fn read_u2(&mut self) {}

    pub(crate) fn skip(&mut self, len: usize) {
        let mut buf = vec![Default::default(); len];
        self.file.read(&mut buf)?;
    }

    fn read(&mut self, len: usize) -> Result<Vec<u8>, Error> {
        let mut buf = vec![Default::default(); len];
        let count = self.file.read(&mut buf).unwrap();
        if count <= 0 {
            Err(Error::new(ErrorKind::Other, "EOF"))
        } else {
            Ok(buf)
        }
    }
}