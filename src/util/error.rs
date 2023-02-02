use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum Error {
    InvalidPath,
    InvalidStringFile,
    InvalidInputPath,
    InvalidSyncType,
    UnsupportedFile,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            Error::InvalidPath => "The provided path is invalid",
            Error::InvalidStringFile => "The provided resource file is invalid",
            Error::InvalidInputPath => "The provided input path is invalid",
            Error::InvalidSyncType => "Unsupported type for sync. sync only supports xml & string files. Checkout the generate command ",
            Error::UnsupportedFile => "Unsupported file type. Supported file types are: csv, xml, strings",
        };

        write!(f, "{message}")
    }
}

impl std::error::Error for Error {}