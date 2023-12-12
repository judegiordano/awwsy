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
