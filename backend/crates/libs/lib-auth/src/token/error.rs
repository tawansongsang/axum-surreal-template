pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    InvalidFormat,
    CannotDecodeIdent,
    CannotDecodeExp,
    HmacFailNewFromSlice,
    SignatureNotMatching,
    ExpNotIso,
    Expired,

    // -- Modules
    InvalidDuration(String)
}

// region:    --- Error Boilerplate
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for Error {}
// endregion: --- Error Boilerplate