use reqwest::Client;
use serde_json as json;

mod error;
pub use crate::error::{Error, Result};

pub mod email;

#[derive(Clone, Debug)]
struct MailerSend {
	pub(crate) http: Client,
	pub(crate) key: String,
}

impl Default for MailerSend {
	fn default() -> Self {
		Self { http: Client::new() }
	}
}

impl MailerSend {
	pub fn new(client: Client) -> Self {
		Self { http: client }
	}

	pub fn email(&self) -> email::Api {
		email::Api::new(self.clone())
	}
}
