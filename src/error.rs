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

	#[error("validation failed: {0:?}")]
	Validation(#[from] ValidationError),

	#[error("request failed: {status:?}: {body:?}")]
  Request {
    status: reqwest::StatusCode,
    body: String,
  }
}

#[derive(Error, Serialize, Deserialize, Debug)]
#[error("validation error: {message}")]
pub struct ValidationError {
	pub message: String,
	pub errors: json::Value,
}
