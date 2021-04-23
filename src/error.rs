use thiserror::Error;
use serde::{Serialize, Deserialize};
use serde_json as json;

pub type Result<T> = ::std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
	#[error("http error: {0}")]
	Reqwest(#[from] reqwest::Error),

	#[error("header error")]
	ReqwestHeader(#[from] reqwest::header::ToStrError),

	#[error("mailersend had an error: {0:?}")]
	MailerSend(#[from] ValidationError),
}

#[derive(Error, Serialize, Deserialize, Debug)]
#[error("validation error: {message}")]
pub struct ValidationError {
	pub message: String,
	pub errors: json::Value,
}
