use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum Error {
    InvalidPath,
    InvalidStringFile
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Oh no, something bad went down")
    }
}

impl std::error::Error for Error {}