use thiserror::Error;

pub type Result<T> = ::std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
	#[error("http error")]
	Reqwest(#[from] reqwest::Error),

	#[error("header error")]
	ReqwestHeader(#[from] reqwest::header::ToStrError),
}
