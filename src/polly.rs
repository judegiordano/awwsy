use crate::{config::CONFIG, types::AwwsyError};
pub use aws_sdk_polly::types::VoiceId;
use aws_sdk_polly::{
    error::{ProvideErrorMetadata, SdkError},
    types::OutputFormat,
    Client,
};
use std::fmt::Debug;

fn map_sdk_error<E: ProvideErrorMetadata + Debug, R: Debug>(err: SdkError<E, R>) -> AwwsyError {
    tracing::error!("[AWS SDK ERROR]: {:?}", err);
    let message = err.message().map_or(err.to_string(), |msg| msg.to_string());
    AwwsyError::PollyError(message)
}

fn map_error(err: impl std::error::Error) -> AwwsyError {
    tracing::error!("[AWWSY ERROR]: {:?}", err);
    AwwsyError::PollyError(err.to_string())
}

pub struct Polly {
    pub client: Client,
}

pub struct TextToSpeechOptions {
    pub voice_id: VoiceId,
}

impl Default for TextToSpeechOptions {
    fn default() -> Self {
        Self {
            voice_id: VoiceId::Amy,
        }
    }
}

impl Polly {
    pub fn new() -> Self {
        Self {
            client: Client::new(&CONFIG),
        }
    }

    pub async fn text_to_speech(
        &self,
        input: impl ToString,
        options: TextToSpeechOptions,
    ) -> Result<Vec<u8>, AwwsyError> {
        let response = self
            .client
            .synthesize_speech()
            .output_format(OutputFormat::Mp3)
            .text(input.to_string())
            .voice_id(options.voice_id)
            .send()
            .await
            .map_err(map_sdk_error)?;
        let buffer = response
            .audio_stream
            .collect()
            .await
            .map_err(map_error)?
            .to_vec();
        Ok(buffer)
    }
}
