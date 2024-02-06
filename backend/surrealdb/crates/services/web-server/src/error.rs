use derive_more::From;
use lib_surrealdb::model;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, From)]
pub enum Error {
    // -- Modules
    #[from]
    Model(model::Error),
}

// region:    --- Error Boilerplate
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for Error {}
// endregion: --- Error Boilerplate
