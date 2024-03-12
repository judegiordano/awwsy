use crate::{config::CONFIG, types::AwwsyError};
use aws_sdk_rekognition::{
	error::{ProvideErrorMetadata, SdkError},
	operation::{
		detect_faces::DetectFacesOutput, detect_moderation_labels::DetectModerationLabelsOutput,
		recognize_celebrities::RecognizeCelebritiesOutput,
	},
	types::Image,
	Client,
};
use std::fmt::Debug;

fn map_sdk_error<E: ProvideErrorMetadata + Debug, R: Debug>(err: SdkError<E, R>) -> AwwsyError {
	tracing::error!("[AWS SDK ERROR]: {:?}", err);
	let message = err.message().map_or(err.to_string(), |msg| msg.to_string());
	AwwsyError::PollyError(message)
}

pub struct Rekognition {
	pub client: Client,
}

impl Rekognition {
	pub async fn new() -> Self {
		Self {
			client: Client::new(CONFIG.get().await),
		}
	}

	fn _build_image(bucket: impl ToString, key: impl ToString) -> Image {
		let s3_obj = aws_sdk_rekognition::types::S3Object::builder()
			.bucket(bucket.to_string())
			.name(key.to_string())
			.build();
		aws_sdk_rekognition::types::Image::builder()
			.s3_object(s3_obj)
			.build()
	}

	pub async fn detect_faces(
		&self,
		bucket: impl ToString,
		key: impl ToString,
	) -> Result<DetectFacesOutput, AwwsyError> {
		self.client
			.detect_faces()
			.image(Self::_build_image(bucket, key))
			.attributes(aws_sdk_rekognition::types::Attribute::All)
			.send()
			.await
			.map_err(map_sdk_error)
	}

	pub async fn detect_celebrities(
		&self,
		bucket: impl ToString,
		key: impl ToString,
	) -> Result<RecognizeCelebritiesOutput, AwwsyError> {
		self.client
			.recognize_celebrities()
			.image(Self::_build_image(bucket, key))
			.send()
			.await
			.map_err(map_sdk_error)
	}

	pub async fn moderate_image(
		&self,
		bucket: impl ToString,
		key: impl ToString,
	) -> Result<DetectModerationLabelsOutput, AwwsyError> {
		self.client
			.detect_moderation_labels()
			.image(Self::_build_image(bucket, key))
			.send()
			.await
			.map_err(map_sdk_error)
	}
}
