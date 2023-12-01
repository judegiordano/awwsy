use crate::config::CONFIG;
use aws_sdk_rekognition::{
    operation::{
        detect_faces::DetectFacesOutput, detect_moderation_labels::DetectModerationLabelsOutput,
        recognize_celebrities::RecognizeCelebritiesOutput,
    },
    primitives::Blob,
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

    fn _build_image(data: impl IntoIterator<Item = u8>) -> Image {
        let blob = Blob::new(data.into_iter().collect::<Vec<u8>>());
        Image::builder().bytes(blob).build()
    }

    pub async fn detect_faces(
        &self,
        image: impl IntoIterator<Item = u8>,
    ) -> Result<DetectFacesOutput, RekognitionError> {
        match self
            .client
            .detect_faces()
            .image(Self::_build_image(image))
            .attributes(aws_sdk_rekognition::types::Attribute::All)
            .send()
            .await
        {
            Ok(output) => Ok(output),
            Err(err) => Err(RekognitionError::DetectFaces(err.to_string())),
        }
    }

    pub async fn detect_celebrities(
        &self,
        image: impl IntoIterator<Item = u8>,
    ) -> Result<RecognizeCelebritiesOutput, RekognitionError> {
        match self
            .client
            .recognize_celebrities()
            .image(Self::_build_image(image))
            .send()
            .await
        {
            Ok(output) => Ok(output),
            Err(err) => Err(RekognitionError::DetectCelebrities(err.to_string())),
        }
    }

    pub async fn moderate_image(
        &self,
        image: impl IntoIterator<Item = u8>,
    ) -> Result<DetectModerationLabelsOutput, RekognitionError> {
        match self
            .client
            .detect_moderation_labels()
            .image(Self::_build_image(image))
            .send()
            .await
        {
            Ok(output) => Ok(output),
            Err(err) => Err(RekognitionError::ModerateImage(err.to_string())),
        }
    }
}
