use std::io;

pub type StlResult<T> = Result<T, StlError>;

#[derive(Debug)]
pub enum StlError {
    IoError(io::Error),
    ParseError(()),
}

impl From<io::Error> for StlError {
    fn from(err: io::Error) -> Self {
        Self::IoError(err)
    }
}

impl From<()> for StlError {
    fn from(_: ()) -> Self {
        Self::ParseError(())
    }
}
