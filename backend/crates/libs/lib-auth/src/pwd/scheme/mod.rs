mod error;
mod scheme_01;
mod scheme_02;

use enum_dispatch::enum_dispatch;

pub use self::error::{Error, Result};

use super::ContentToHash;

pub const DEFAULT_SCHEME: &str = "02";

#[derive(Debug)]
pub enum SchemeStatus {
    Ok,         // The pwd uses the latest scheme. All good.
    Outdated,   // The pwd uses and old scheme.
}

#[enum_dispatch]
pub trait Scheme {
    fn hash(&self, to_hash: &ContentToHash) -> Result<String>;

    fn validate(&self, to_hash: &ContentToHash, pwd_ref: &str) -> Result<()>;
}

#[enum_dispatch(Scheme)]
pub enum SchemeDispatcher {
    Scheme01(scheme_01::Scheme01),
    // Scheme02(scheme_02::Scheme02),

}