
use std::{env, str::FromStr};

use crate::b64::b64u_decode;

pub fn get_env(name: &'static str) -> Result<String> {
	env::var(name).map_err(|_| Error::MissingEnv(name))
}

pub fn get_env_parse<T: FromStr>(name: &'static str) -> Result<T> {
	let val = get_env(name)?;
	val.parse::<T>().map_err(|_| Error::WrongFormat(name))
}

pub fn get_env_b64u_as_u8s(name: &'static str) -> Result<Vec<u8>> {
	b64u_decode(&get_env(name)?).map_err(|_| Error::WrongFormat(name))
}


// region:		--- Error
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
	MissingEnv(&'static str),
	WrongFormat(&'static str),
}

// region:		--- Error Boilerplate
impl std::fmt::Display for Error {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
		write!(f, "{self:?}")
	}
}

impl std::error::Error for Error {}
// endregion:	--- Error Boilerplate

// endregion:	--- Error