use serde::{Serialize, Deserialize};

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

pub mod send {
	use serde::Serialize;

	#[derive(Clone, Serialize, Debug)]
	#[serde(untagged)]
	pub enum Request {
		Message(Message),
		Template(Template),
	}

	#[derive(Clone, Serialize, Debug)]
	pub struct Message {
		pub from: super::Recipient,
		pub to: Vec<super::Recipient>,
		#[serde(default)]
		pub cc: Vec<super::Recipient>,
		#[serde(default)]
		pub bcc: Vec<super::Recipient>,
		#[serde(default)]
		pub reply_to: Option<super::Recipient>,
		pub subject: String,
		pub text: String,
		pub html: String,
		#[serde(default)]
		pub attachments: Vec<super::Attachment>,
		#[serde(default)]
		pub tags: Vec<String>,
		#[serde(default)]
		pub variables: Vec<super::Variable>,
	}

	#[derive(Clone, Serialize, Debug)]
	pub struct Template {
		pub to: Vec<super::Recipient>,
		#[serde(default)]
		pub cc: Vec<super::Recipient>,
		#[serde(default)]
		pub bcc: Vec<super::Recipient>,
		#[serde(default)]
		pub reply_to: Option<super::Recipient>,
		pub template_id: String,
		#[serde(default)]
		pub attachments: Vec<super::Attachment>,
		#[serde(default)]
		pub tags: Vec<String>,
		#[serde(default)]
		pub variables: Vec<super::Variable>,
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

	pub async fn send(&self, request: send::Request) -> crate::Result<MessageId> {
		let response = self.mailer.http.post("https://api.mailersend.com/v1/email")
			.bearer_auth(&self.mailer.key)
			.json(&request)
			.send()
			.await?
			.error_for_status()?;

		Ok(response.headers()["X-Message-Id"].to_str()?.into())
	}

	pub async fn message(&self, request: send::Message) -> crate::Result<MessageId> {
		self.send(send::Request::Message(request)).await
	}

	pub async fn template(&self, request: send::Template) -> crate::Result<MessageId> {
		self.send(send::Request::Template(request)).await
	}
}
