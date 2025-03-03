use core::fmt::Display;
use postcard::Error as PostcardError;

// Error handling example (add to your errors.rs)
#[derive(Debug)]
pub enum Error {
    Serialization(PostcardError),
    Deserialization(PostcardError),
    Encoding,
    Decoding,
    CrcMismatch,
    FrameTooLarge,
    InvalidFrame,
    VersionMismatch,

    PostcardError(postcard::Error),
    TryFromSliceError(core::array::TryFromSliceError),
}

impl From<PostcardError> for Error {
    fn from(err: postcard::Error) -> Self {
        Error::PostcardError(err)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Error::PostcardError(err) => write!(f, "Postcard error: {:?}", err),
            Error::TryFromSliceError(err) => write!(f, "TryFromSlice error: {:?}", err),
            _ => write!(f, "Unkown error"),
        }
    }
}
