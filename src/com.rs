use btoi;
use futures::sync::mpsc::SendError;
use std::convert::From;
use std::io;
use std::result;

#[derive(Debug)]
pub enum Error {
    None,
    MoreData,
    BadMsg,
    BadKey,
    BadCmd,
    IoError(io::Error),
    Critical,
    ParseIntError(btoi::ParseIntegerError),
    SendError(SendError<::Resp>),
}

impl From<SendError<::Resp>> for Error {
    fn from(oe: SendError<::Resp>) -> Error {
        Error::SendError(oe)
    }
}

impl From<io::Error> for Error {
    fn from(oe: io::Error) -> Error {
        Error::IoError(oe)
    }
}

impl From<btoi::ParseIntegerError> for Error {
    fn from(oe: btoi::ParseIntegerError) -> Error {
        Error::ParseIntError(oe)
    }
}

pub type AsResult<T> = result::Result<T, Error>;

const LOWER_BEGIN: u8 = 'a' as u8;
const LOWER_END: u8 = 'z' as u8;
const UPPER_TO_LOWER: u8 = 'a' as u8 - 'A' as u8;

pub fn update_to_upper(src: &mut [u8]) {
    for b in src {
        if *b < LOWER_BEGIN || *b > LOWER_END {
            continue;
        }
        *b = *b - UPPER_TO_LOWER;
    }
}