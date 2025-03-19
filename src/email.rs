use serde::{Serialize, Deserialize};
use serde_json as json;
use crate::error;

pub type MessageId = String;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Recipient {
	pub email: String,
	pub name: Option<String>,
}

#[derive(Clone, Serialize, Debug)]
pub struct Attachment {
	pub id: Option<String>,
	pub filename: String,
	pub content: String,
}

#[derive(Clone, Serialize, Debug)]
pub struct Variable {
	pub email: String,
	pub substitutions: Vec<Substitution>,
}

#[derive(Clone, Serialize, Debug)]
pub struct Substitution {
	pub var: String,
	pub value: String,
}

#[derive(Clone, Serialize, Debug)]
pub struct Message {
	pub from: Recipient,
	pub to: Vec<Recipient>,
	#[serde(default)]
	pub cc: Vec<Recipient>,
	#[serde(default)]
	pub bcc: Vec<Recipient>,
	#[serde(default)]
	pub reply_to: Option<Recipient>,
	pub subject: String,
	pub text: String,
	pub html: String,
	#[serde(default)]
	pub attachments: Vec<Attachment>,
	#[serde(default)]
	pub tags: Vec<String>,
	#[serde(default)]
	pub variables: Vec<Variable>,
}

#[derive(Clone, Serialize, Debug)]
pub struct Template {
	pub to: Vec<Recipient>,
	#[serde(default)]
	pub cc: Vec<Recipient>,
	#[serde(default)]
	pub bcc: Vec<Recipient>,
	#[serde(default)]
	pub reply_to: Option<Recipient>,
	pub template_id: String,
	#[serde(default)]
	pub attachments: Vec<Attachment>,
	#[serde(default)]
	pub tags: Vec<String>,
	#[serde(default)]
	pub variables: Vec<Variable>,
}

pub mod send {
	use serde::Serialize;

	#[derive(Clone, Serialize, Debug)]
	#[serde(untagged)]
	pub enum Request {
		Message(super::Message),
		Template(super::Template),
	}
}

#[derive(Clone, Debug)]
pub struct Api {
	mailer: crate::MailerSend,
}

impl Api {
	pub fn new(mailer: crate::MailerSend) -> Self {
		Self { mailer }
	}

	pub async fn verify_email(&self, email: &str) -> crate::Result<json::Value> {
		let response = self.mailer.http.post("https://api.mailersend.com/v1/email-verification/verify")
			.bearer_auth(&self.mailer.key)
			.json(&json::json!({ "email": email }))
			.send()
			.await?;

		if response.status().is_success() {
			let decoded_response = response.json::<json::Value>().await?;
			Ok(decoded_response)
		} else {
			let status = response.status();
			let body = response.text().await?;
			Err(error::Error::Request { status, body })
		}
	}

	pub async fn send(&self, request: send::Request) -> crate::Result<MessageId> {
		let response = self.mailer.http.post("https://api.mailersend.com/v1/email")
			.bearer_auth(&self.mailer.key)
			.json(&request)
			.send()
			.await?;

		if response.status().is_success() {
			if let Some(x_messgae_id) = response.headers().get("X-Message-Id") {
				return Ok(x_messgae_id.to_str()?.into());
			}

			return Ok("InvalidMessageId".into());
		}

		let status = response.status();
	    let body = response.text().await?;

		if let Ok(err) = json::from_str::<error::ValidationError>(&body) {
			Err(err.into())
		} else {
			Err(error::Error::Request { status, body })
		}
	}

	pub async fn message(&self, request: Message) -> crate::Result<MessageId> {
		self.send(send::Request::Message(request)).await
	}

	pub async fn template(&self, request: Template) -> crate::Result<MessageId> {
		self.send(send::Request::Template(request)).await
	}
}
