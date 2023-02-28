use std::io::Error;

#[derive(Debug)]
pub struct EndOfFile;

impl std::fmt::Display for EndOfFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Reached end of file")
    }
}

impl std::error::Error for EndOfFile {}

impl From<EndOfFile> for Error {
    fn from(eof: EndOfFile) -> Error {
        Error::new(std::io::ErrorKind::Other, eof)
    }
}

#[derive(Debug)]
pub struct NotSupport;

impl std::fmt::Display for NotSupport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Not support")
    }
}

impl std::error::Error for NotSupport {}

impl From<NotSupport> for Error {
    fn from(eof: NotSupport) -> Error {
        Error::new(std::io::ErrorKind::Other, eof)
    }
}