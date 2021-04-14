use thiserror::Error;

pub type Result<T> = ::std::result::Result<T, Error>;

#[derive(Clone, Error, Debug)]
pub enum Error {
	#[error("http error: {}")]
	Reqwest(#[from] reqwest::Error),
}
