use crate::{config::CONFIG, types::AwwsyError};
use aws_sdk_s3::{
	error::{ProvideErrorMetadata, SdkError},
	operation::{
		delete_object::DeleteObjectOutput, get_object::GetObjectOutput,
		list_buckets::ListBucketsOutput, list_objects::ListObjectsOutput,
		put_object::PutObjectOutput,
	},
	presigning::PresigningConfig,
	Client,
};
use std::{fmt::Debug, time::Duration};

fn map_sdk_error<E: ProvideErrorMetadata + Debug, R: Debug>(err: SdkError<E, R>) -> AwwsyError {
	tracing::error!("[AWS SDK ERROR]: {:?}", err);
	let message = err.message().map_or(err.to_string(), |msg| msg.to_string());
	AwwsyError::PollyError(message)
}

fn map_error(err: impl std::error::Error) -> AwwsyError {
	tracing::error!("[AWWSY ERROR]: {:?}", err);
	AwwsyError::PollyError(err.to_string())
}

pub async fn list_buckets() -> Result<ListBucketsOutput, AwwsyError> {
	let client = Client::new(&CONFIG);
	client.list_buckets().send().await.map_err(map_sdk_error)
}

pub struct Bucket {
	pub name: String,
	pub client: Client,
}

impl Bucket {
	fn _build_presigned_config(duration: Duration) -> Result<PresigningConfig, AwwsyError> {
		PresigningConfig::expires_in(duration).map_err(map_error)
	}

	pub fn new(bucket_name: impl ToString) -> Self {
		Self {
			name: bucket_name.to_string(),
			client: Client::new(&CONFIG),
		}
	}

	pub fn name(&self) -> String {
		self.name.to_string()
	}

	pub async fn get_object(&self, key: impl ToString) -> Result<GetObjectOutput, AwwsyError> {
		self.client
			.get_object()
			.bucket(&self.name)
			.key(key.to_string())
			.send()
			.await
			.map_err(map_sdk_error)
	}

	pub async fn list_objects(&self) -> Result<ListObjectsOutput, AwwsyError> {
		self.client
			.list_objects()
			.bucket(&self.name)
			.send()
			.await
			.map_err(map_sdk_error)
	}

	pub async fn put_object(
		&self,
		key: impl ToString,
		body: impl IntoIterator<Item = u8>,
	) -> Result<PutObjectOutput, AwwsyError> {
		let stream = body.into_iter().collect::<Vec<_>>();
		self.client
			.put_object()
			.bucket(&self.name)
			.body(stream.into())
			.key(key.to_string())
			.send()
			.await
			.map_err(map_sdk_error)
	}

	pub async fn delete_object(
		&self,
		key: impl ToString,
	) -> Result<DeleteObjectOutput, AwwsyError> {
		self.client
			.delete_object()
			.bucket(&self.name)
			.key(key.to_string())
			.send()
			.await
			.map_err(map_sdk_error)
	}

	pub async fn get_presigned_url(
		&self,
		key: impl ToString,
		expires_in: Duration,
	) -> Result<String, AwwsyError> {
		let request = self
			.client
			.get_object()
			.bucket(&self.name)
			.key(key.to_string())
			.presigned(Self::_build_presigned_config(expires_in)?)
			.await
			.map_err(map_sdk_error)?;
		Ok(request.uri().to_string())
	}

	pub async fn put_presigned_url(
		&self,
		key: impl ToString,
		expires_in: Duration,
	) -> Result<String, AwwsyError> {
		let request = self
			.client
			.put_object()
			.bucket(&self.name)
			.key(key.to_string())
			.presigned(Self::_build_presigned_config(expires_in)?)
			.await
			.map_err(map_sdk_error)?;
		Ok(request.uri().to_string())
	}
}
