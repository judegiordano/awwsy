use std::fmt::Debug;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AwwsyError {
	#[error("AWS SDK Service Error: {0}")]
	ServiceError(String),
	#[error("AWS SDK Polly Error: {0}")]
	PollyError(String),
	#[error("AWS SDK Rekognition Error: {0}")]
	RekognitionError(String),
	#[error("AWS SDK S3 Error: {0}")]
	S3Error(String),
	#[error("AWS SDK SQS Error: {0}")]
	SQSError(String),
}

impl AwwsyError {
	pub fn service(err: impl std::error::Error) -> AwwsyError {
		tracing::error!("[AWWSY SERVICE ERROR]: {:?}", err);
		AwwsyError::ServiceError(err.to_string())
	}

	pub fn polly(err: impl std::error::Error) -> AwwsyError {
		tracing::error!("[AWWSY POLLY ERROR]: {:?}", err);
		AwwsyError::PollyError(err.to_string())
	}

	pub fn rekognition(err: impl std::error::Error) -> AwwsyError {
		tracing::error!("[AWWSY REKOGNITION ERROR]: {:?}", err);
		AwwsyError::RekognitionError(err.to_string())
	}

	pub fn s3(err: impl std::error::Error) -> AwwsyError {
		tracing::error!("[AWWSY S3 ERROR]: {:?}", err);
		AwwsyError::S3Error(err.to_string())
	}

	pub fn sqs(err: impl std::error::Error) -> AwwsyError {
		tracing::error!("[AWWSY SQS ERROR]: {:?}", err);
		AwwsyError::SQSError(err.to_string())
	}
}
