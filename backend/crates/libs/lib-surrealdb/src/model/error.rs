use super::store;
use derive_more::From;
use serde::Serialize;
use serde_with::{serde_as, DisplayFromStr};

pub type Result<T> = std::result::Result<T, Error>;

#[serde_as]
#[derive(Debug, Serialize, From)]
pub enum Error {
    CannotComparePasswordFromDB,
    DataNotFound,
    DataNotFoundFromCreated,
    DataNotFoundFromDelete,
    DataNotFoundFromUpdate,
    UserIdNotFound,

    // -- Modules
    #[from]
    Store(store::Error),

    // -- Externals
    #[from]
    Surrealdb(#[serde_as(as = "DisplayFromStr")] surrealdb::Error),
}

// region:    --- Error Boilerplate
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for Error {}
// endregion: --- Error Boilerplate
