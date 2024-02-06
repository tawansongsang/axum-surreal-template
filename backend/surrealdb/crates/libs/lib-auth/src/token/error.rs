use serde::Serialize;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Serialize)]
pub enum Error {
    HmacFailNewFromSlice,

    InvalidFormat,
    CannotDecodeIdent,
    CannotDecodeExp,
    SignatureNotMatching,
    ExpNotIso,
    Expired,

    // -- Modules
    InvalidDuration(String),
}

// region:    --- Error Boilerplate
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for Error {}
// endregion: --- Error Boilerplate
