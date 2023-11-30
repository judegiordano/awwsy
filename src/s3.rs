use aws_sdk_s3::{
    error::SdkError,
    operation::{get_object::GetObjectOutput, list_buckets::ListBucketsOutput},
    Client,
};
use thiserror::Error;

use crate::config;

#[derive(Debug, Error)]
pub enum S3Error {
    #[error("{0}")]
    ListBuckets(String),
    #[error("{0}")]
    GetObject(String),
}

pub async fn list_buckets() -> Result<ListBucketsOutput, S3Error> {
    let config = config::Config::new().await;
    let client = Client::new(&config);
    match client.list_buckets().send().await {
        Ok(output) => Ok(output),
        Err(err) => {
            tracing::error!("error listing s3 buckets {:?}", err);
            return Err(S3Error::ListBuckets(err.to_string()));
        }
    }
}

pub struct Bucket {
    name: String,
    client: Client,
}

impl Bucket {
    pub async fn new(bucket_name: impl ToString) -> Self {
        let config = config::Config::new().await;
        let client = Client::new(&config);
        Self {
            name: bucket_name.to_string(),
            client,
        }
    }

    pub async fn get_object(&self, key: String) -> Result<GetObjectOutput, S3Error> {
        let Self { client, name } = self;
        let request = client.get_object().bucket(name).key(key);
        match request.send().await {
            Ok(output) => Ok(output),
            Err(err) => {
                tracing::error!("error getting object {:?}", err);
                return Err(S3Error::GetObject(err.to_string()));
            }
        }
    }
}
