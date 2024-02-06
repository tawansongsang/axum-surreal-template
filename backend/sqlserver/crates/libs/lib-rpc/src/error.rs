use derive_more::From;
use lib_sqlserver::model;
use serde::Serialize;
use serde_with::{serde_as, DisplayFromStr};

pub type Result<T> = std::result::Result<T, Error>;

#[serde_as]
#[derive(Debug, Serialize, From)]
pub enum Error {
    MissingCtx,

    // -- RPC Router
    RpcMethodUnknow(String),
    RpcIntoParamsMissing,

    // -- Internal Modules
    #[from]
    Model(model::Error),

    // -- External Modules
    #[from]
    SerdeJson(#[serde_as(as = "DisplayFromStr")] serde_json::Error),
}

// region:    --- Error Boilerlate
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for Error {}
// endregion: --- Error Boilerplate
