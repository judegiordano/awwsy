use aws_sdk_sqs::{self, operation::send_message::SendMessageOutput, Client};
use serde::Serialize;
use thiserror::Error;

use crate::config::CONFIG;

#[derive(Debug, Error)]
pub enum SQSError {
    #[error("error serializing message: {0}")]
    Serialize(String),
    #[error("error sending message: {0}")]
    SendMessage(String),
    #[error("error sending fifo message: {0}")]
    SendFifoMessage(String),
}

pub struct SqsMessageOptions {
    delay_seconds: i32,
}

impl Default for SqsMessageOptions {
    fn default() -> Self {
        Self { delay_seconds: 0 }
    }
}

pub struct SqsFifoMessageOptions {
    message_group_id: String,
    message_deduplication_id: String,
}

impl Default for SqsFifoMessageOptions {
    fn default() -> Self {
        Self {
            message_group_id: String::new(),
            message_deduplication_id: String::new(),
        }
    }
}

pub struct Queue {
    queue_url: String,
    client: Client,
}

impl Queue {
    fn _serialize_body(message: impl Serialize) -> Result<String, SQSError> {
        match serde_json::to_string(&message) {
            Ok(string) => Ok(string),
            Err(err) => Err(SQSError::Serialize(err.to_string())),
        }
    }

    pub fn new(queue_url: impl ToString) -> Self {
        Self {
            queue_url: queue_url.to_string(),
            client: Client::new(&CONFIG),
        }
    }

    pub fn queue_url(&self) -> String {
        self.queue_url.to_string()
    }

    pub async fn send_message(
        &self,
        message: impl Serialize,
        options: SqsMessageOptions,
    ) -> Result<SendMessageOutput, SQSError> {
        let Self { client, queue_url } = self;
        let response = client
            .send_message()
            .queue_url(queue_url)
            .delay_seconds(options.delay_seconds)
            .message_body(Self::_serialize_body(message)?);
        match response.send().await {
            Ok(output) => Ok(output),
            Err(err) => Err(SQSError::SendMessage(err.to_string())),
        }
    }

    pub async fn send_fifo_message(
        &self,
        message: impl Serialize,
        options: SqsFifoMessageOptions,
    ) -> Result<SendMessageOutput, SQSError> {
        let Self { client, queue_url } = self;
        let response = client
            .send_message()
            .queue_url(queue_url)
            .message_deduplication_id(options.message_deduplication_id)
            .message_group_id(options.message_group_id)
            .message_body(Self::_serialize_body(message)?);
        match response.send().await {
            Ok(output) => Ok(output),
            Err(err) => Err(SQSError::SendFifoMessage(err.to_string())),
        }
    }
}