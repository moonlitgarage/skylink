use core::fmt::Display;

#[derive(Debug)]
pub enum SkylinkError {
    // Serialization(PostcardError),
    // Deserialization(PostcardError),
    Encoding,
    Decoding,
    CrcMismatch,
    FrameTooLarge,
    InvalidFrame,
    VersionMismatch,
    UnkownMessageType,

    // PostcardError(postcard::Error),
    TryFromSliceError(core::array::TryFromSliceError),
}

// impl From<PostcardError> for SkylinkError {
//     fn from(err: postcard::Error) -> Self {
//         SkylinkError::PostcardError(err)
//     }
// }

impl Display for SkylinkError {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            // SkylinkError::PostcardError(err) => write!(f, "Postcard error: {:?}", err),
            SkylinkError::TryFromSliceError(err) => write!(f, "TryFromSlice error: {:?}", err),
            _ => write!(f, "Unkown error"),
        }
    }
}
