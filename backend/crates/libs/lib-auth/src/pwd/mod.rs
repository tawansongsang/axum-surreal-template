mod error;
mod scheme;

use uuid::Uuid;

pub use self::error::{Error, Result};

// region:    --- Types
/// The clean content to hash, with the salt.
///
/// Notes:
///    - Since content is sensitive information, we do NOT implement default debug for this struct.
///    - The clone is only implement for testing
#[cfg_attr(test, derive(Clone))]
pub struct ContentToHash {
    pub content: String, // Clear content.
    pub salt: Uuid
}
// endregion: --- Types

/// Hash the password with the default scheme.
pub async fn hash_pwd(to_hash: ContentToHash) -> Result<String> {
	tokio::task::spawn_blocking(move || hash_for_scheme(DEFAULT_SCHEME, to_hash))
		.await
		.map_err(|_| Error::FailSpawnBlockForHash)?
}

fn hash_for_scheme(scheme_name: &str, to_hash: ContentToHash) -> Result<String> {
    // let pwd_hashed = get_scheme(scheme_name).unwrap().hash(&to_hash).unwrap();

    todo!()
}