// TODO: @tomorrow function for format thai timezone
use derive_more::From;
use serde::Serialize;
use serde_with::{serde_as, DisplayFromStr};
use time::{OffsetDateTime, error::Format, Duration};

pub use time::format_description::well_known::Rfc3339;

pub fn now_utc() -> OffsetDateTime {
	OffsetDateTime::now_utc()
}

pub fn format_time(time: OffsetDateTime) -> Result<String> {
	let ft = time.format(&Rfc3339)?;
	Ok(ft)
}

pub fn now_utc_plus_sec_str(sec: f64) -> Result<String> {
	let new_time = now_utc() + Duration::seconds_f64(sec);
	let ft = format_time(new_time)?;
	Ok(ft)
}

pub fn parse_utc(moment: &str) -> Result<OffsetDateTime> {
	OffsetDateTime::parse(moment, &Rfc3339)
		.map_err(|_| Error::FailToDateParse(moment.to_string()))
}


// region:		--- Error
pub type Result<T> = std::result::Result<T, Error>;

#[serde_as]
#[derive(Debug, Serialize, From)]
pub enum Error {
	FailToDateParse(String),
	// -- Externals
	#[from]
	TimeFormat(#[serde_as(as = "DisplayFromStr")] Format),
}

// region:		--- Error Boilerplate
impl std::fmt::Display for Error {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
	    write!(f, "{self:?}")
	}
}

impl std::error::Error for Error {}
// endregion:	--- Error Boilerplate

// endregion:	--- Error