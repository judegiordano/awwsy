use crate::{config::CONFIG, types::AwwsyError};
use aws_lambda_events::sqs::SqsMessage;
use aws_sdk_sqs::{
	self,
	error::{ProvideErrorMetadata, SdkError},
	operation::send_message::SendMessageOutput,
	Client,
};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

fn map_sdk_error<E: ProvideErrorMetadata + Debug, R: Debug>(err: SdkError<E, R>) -> AwwsyError {
	tracing::error!("[AWS SDK ERROR]: {:?}", err);
	let message = err.message().map_or(err.to_string(), |msg| msg.to_string());
	AwwsyError::PollyError(message)
}

pub fn parse_records<T: for<'a> Deserialize<'a>>(
	records: impl IntoIterator<Item = SqsMessage>,
) -> Result<Vec<T>, AwwsyError> {
	let mut items = vec![];
	for record in records {
		if let Some(body) = record.body {
			items.push(serde_json::from_str::<T>(&body).map_err(AwwsyError::sqs)?)
		}
	}
	Ok(items)
}

pub struct SqsMessageOptions {
	pub delay_seconds: i32,
}

impl Default for SqsMessageOptions {
	fn default() -> Self {
		Self { delay_seconds: 0 }
	}
}

pub struct SqsFifoMessageOptions {
	pub message_group_id: String,
	pub message_deduplication_id: String,
}

impl Default for SqsFifoMessageOptions {
	fn default() -> Self {
		Self {
			message_group_id: nanoid::nanoid!(),
			message_deduplication_id: nanoid::nanoid!(),
		}
	}
}

pub struct Queue {
	queue_url: String,
	client: Client,
}

impl Queue {
	fn _serialize_body(message: impl Serialize) -> Result<String, AwwsyError> {
		serde_json::to_string(&message).map_err(AwwsyError::sqs)
	}

	pub async fn new(queue_url: impl ToString) -> Self {
		Self {
			queue_url: queue_url.to_string(),
			client: Client::new(CONFIG.get().await),
		}
	}

	pub fn queue_url(&self) -> String {
		self.queue_url.to_string()
	}

	pub async fn send_message(
		&self,
		message: impl Serialize,
		options: SqsMessageOptions,
	) -> Result<SendMessageOutput, AwwsyError> {
		self.client
			.send_message()
			.queue_url(&self.queue_url)
			.delay_seconds(options.delay_seconds)
			.message_body(Self::_serialize_body(message)?)
			.send()
			.await
			.map_err(map_sdk_error)
	}

	pub async fn send_fifo_message(
		&self,
		message: impl Serialize,
		options: SqsFifoMessageOptions,
	) -> Result<SendMessageOutput, AwwsyError> {
		self.client
			.send_message()
			.queue_url(&self.queue_url)
			.message_deduplication_id(options.message_deduplication_id)
			.message_group_id(options.message_group_id)
			.message_body(Self::_serialize_body(message)?)
			.send()
			.await
			.map_err(map_sdk_error)
	}
}
