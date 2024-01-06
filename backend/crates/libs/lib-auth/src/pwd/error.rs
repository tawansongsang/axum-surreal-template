use super::scheme;
use derive_more::From;
use serde::Serialize;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Serialize, From)]
pub enum Error {
    FailSpawnBlockForHash,
    PwdWithSchemeFailedParse,
    FailSpawnBlockForValidate,

    // -- Modules
    #[from]
    Scheme(scheme::Error),
}

// region:    --- Error Boilerplate
impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}
// endregion: --- Error Boilerplate
