mod deserializer;
mod serializer;

pub use self::{deserializer::*, serializer::*};

use std::fmt::{self, Display};

use serde::{de, ser};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Debug)]
pub enum Error {
    Message(String),
    Eof,
}

impl ser::Error for Error {
    fn custom<T: Display>(msg: T) -> Error {
        Error::Message(msg.to_string())
    }
}

impl de::Error for Error {
    fn custom<T: Display>(msg: T) -> Error {
        Error::Message(msg.to_string())
    }
}

impl Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Message(msg) => formatter.write_str(msg),
            Error::Eof => formatter.write_str(
                "unexpected end of file, some data in your save are unexpected or your save is corrupted ?\n\
                Save again and retry. If this error persists, please report a bug with your save attached"),
        }
    }
}

impl std::error::Error for Error {}
