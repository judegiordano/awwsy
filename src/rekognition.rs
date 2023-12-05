use crate::config::CONFIG;
use aws_sdk_rekognition::{
    operation::{
        detect_faces::DetectFacesOutput, detect_moderation_labels::DetectModerationLabelsOutput,
        recognize_celebrities::RecognizeCelebritiesOutput,
    },
    types::Image,
    Client,
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RekognitionError {
    #[error("error detecting faces: {0}")]
    DetectFaces(String),
    #[error("error detecting celebrities: {0}")]
    DetectCelebrities(String),
    #[error("error moderating image: {0}")]
    ModerateImage(String),
}

pub struct Rekognition {
    client: Client,
}

impl Rekognition {
    pub fn new() -> Self {
        Self {
            client: Client::new(&CONFIG),
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
    ) -> Result<DetectFacesOutput, RekognitionError> {
        match self
            .client
            .detect_faces()
            .image(Self::_build_image(bucket, key))
            .attributes(aws_sdk_rekognition::types::Attribute::All)
            .send()
            .await
        {
            Ok(output) => Ok(output),
            Err(err) => {
                tracing::error!("{:?}", err);
                Err(RekognitionError::DetectFaces(err.to_string()))
            }
        }
    }

    pub async fn detect_celebrities(
        &self,
        bucket: impl ToString,
        key: impl ToString,
    ) -> Result<RecognizeCelebritiesOutput, RekognitionError> {
        match self
            .client
            .recognize_celebrities()
            .image(Self::_build_image(bucket, key))
            .send()
            .await
        {
            Ok(output) => Ok(output),
            Err(err) => {
                tracing::error!("{:?}", err);
                Err(RekognitionError::DetectCelebrities(err.to_string()))
            }
        }
    }

    pub async fn moderate_image(
        &self,
        bucket: impl ToString,
        key: impl ToString,
    ) -> Result<DetectModerationLabelsOutput, RekognitionError> {
        match self
            .client
            .detect_moderation_labels()
            .image(Self::_build_image(bucket, key))
            .send()
            .await
        {
            Ok(output) => Ok(output),
            Err(err) => {
                tracing::error!("{:?}", err);
                Err(RekognitionError::ModerateImage(err.to_string()))
            }
        }
    }
}
