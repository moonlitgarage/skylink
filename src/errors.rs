use core::fmt::Display;

#[derive(Debug)]
pub enum Error {
    PostcardError(postcard::Error),
    TryFromSliceError(core::array::TryFromSliceError),
}

impl From<postcard::Error> for Error {
    fn from(err: postcard::Error) -> Self {
        Error::PostcardError(err)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Error::PostcardError(err) => write!(f, "Postcard error: {:?}", err),
            Error::TryFromSliceError(err) => write!(f, "TryFromSlice error: {:?}", err),
        }
    }
}
