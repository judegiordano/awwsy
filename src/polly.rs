use crate::config::CONFIG;
pub use aws_sdk_polly::types::VoiceId;
use aws_sdk_polly::{types::OutputFormat, Client};
use std::fmt::Debug;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PollyError {
    #[error("error synthesizing text to speech: {0}")]
    TextToSpeech(String),
}

pub struct Polly {
    client: Client,
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
    ) -> Result<Vec<u8>, PollyError> {
        let response = self
            .client
            .synthesize_speech()
            .output_format(OutputFormat::Mp3)
            .text(input.to_string())
            .voice_id(options.voice_id);
        let response = match response.send().await {
            Ok(response) => response,
            Err(err) => {
                tracing::error!("{:?}", err);
                return Err(PollyError::TextToSpeech(err.to_string()));
            }
        };
        match response.audio_stream.collect().await {
            Ok(data) => Ok(data.to_vec()),
            Err(err) => {
                tracing::error!("{:?}", err);
                return Err(PollyError::TextToSpeech(err.to_string()));
            }
        }
    }
}
