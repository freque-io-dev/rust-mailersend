use reqwest::Client;

mod error;
pub use crate::error::{Error, Result};

pub mod email;

#[derive(Clone, Debug)]
pub struct MailerSend {
	pub(crate) http: Client,
	pub(crate) key: String,
}

impl MailerSend {
	pub fn new(key: String) -> Self {
		Self { http: Client::new(), key }
	}

	pub fn with_client(client: Client, key: String) -> Self {
		Self { http: client, key }
	}

	pub fn email(&self) -> email::Api {
		email::Api::new(self.clone())
	}
}
