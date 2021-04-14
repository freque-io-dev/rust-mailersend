use reqwest::Client;
use serde::{Serialize, Deserialize};

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

pub mod send {
	use serde::{Serialize, Deserialize};

	#[derive(Clone, Serialize, Debug)]
	#[serde(untagged)]
	pub enum Request {
		Message(Message),
		Template(Template),
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
		pub text: String
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
}

#[derive(Clone, Debug)]
pub struct Api {
	mailer: crate::MailerSend,
}

impl Api {
	pub async fn send(&self, request: send::Request) -> crate::Result<()> {
		let response = self.mailer.post("https://api.mailersend.com/v1/email")
			.bearer_auth(&self.mailer.key)
			.json(&request)
			.send()
			.await?
			.error_for_status()?;

		Ok(())
	}

	pub async fn message(&self, request: send::Message) -> crate::Result<()> {
		self.send(send::Request::Message(request)).await
	}

	pub async fn template(&self, request: send::Template) -> crate::Result<()> {
		self.send(send::Request::Template(request)).await
	}
}
