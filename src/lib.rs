pub mod config;

#[cfg(feature = "lambda_events")]
pub use aws_lambda_events as lambda_events;

#[cfg(feature = "lambda_runtime")]
pub use lambda_runtime;

#[cfg(feature = "s3")]
pub mod s3;

#[cfg(feature = "sqs")]
pub mod sqs;

#[cfg(feature = "rekognition")]
pub mod rekognition;

#[cfg(feature = "polly")]
pub mod polly;

#[cfg(test)]
mod tests;
