use serde::Serialize;
use serde_with::{serde_as, DisplayFromStr};

use super::store;

pub type Result<T> = std::result::Result<T, Error>;

#[serde_as]
#[derive(Debug, derive_more::From, Serialize)]
pub enum Error {
    // -- Modules
    #[from]
    Store(store::Error),

    // -- Externals
    #[from]
    Bb8(#[serde_as(as = "DisplayFromStr")] bb8::RunError<bb8_tiberius::Error>),
    #[from]
    Tiberius(#[serde_as(as = "DisplayFromStr")] tiberius::error::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for Error {}
