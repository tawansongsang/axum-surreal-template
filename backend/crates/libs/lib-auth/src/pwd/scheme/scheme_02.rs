// TODO: Implement Scheme02 with argon2 and fixed scheme02 error hash_password
use std::sync::OnceLock;

use super::{Error, Result};
use crate::config::auth_config;
use crate::pwd::{scheme::Scheme, ContentToHash};
use argon2::password_hash::SaltString;
use argon2::{Algorithm, Argon2, Params, Version};

pub struct Scheme02;

impl Scheme for Scheme02 {
    fn hash(&self, to_hash: &ContentToHash) -> Result<String> {
        let argon2 = get_argon2();

        let salt_b64 = SaltString::encode_b64(to_hash.salt.as_bytes()).map_err(|_| Error::Salt)?;

        let pwd = argon2
            .hash_password(to_hash.content.as_bytes(), &salt_b64)
            .map_err(|_| Error::Hash)?
            .to_string();

        Ok(pwd)
    }

    fn validate(&self, to_hash: &ContentToHash, pwd_ref: &str) -> Result<()> {
        todo!()
    }
}

fn get_argon2() -> &'static Argon2<'static> {
    static INSTANCE: OnceLock<Argon2<'static>> = OnceLock::new();

    INSTANCE.get_or_init(|| {
        let key = &auth_config().PWD_KEY;
        Argon2::new_with_secret(
            key,
            Algorithm::Argon2id, // Same as Argon::default()
            Version::V0x13,      // Same as Argon2::default()
            Params::default(),
        )
        .unwrap() // TODO: needs to fail early
    })
}
