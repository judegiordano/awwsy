use crate::config;
use aws_sdk_s3::{
    operation::{
        delete_object::DeleteObjectOutput, get_object::GetObjectOutput,
        list_buckets::ListBucketsOutput, put_object::PutObjectOutput,
    },
    presigning::PresigningConfig,
    Client,
};
use std::time::Duration;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum S3Error {
    #[error("error listing s3 buckets: {0}")]
    ListBuckets(String),
    #[error("error getting object {0}")]
    GetObject(String),
    #[error("error putting object: {0}")]
    PutObject(String),
    #[error("error deleting object: {0}")]
    DeleteObject(String),
    #[error("error creating presigned config: {0}")]
    PresignedConfig(String),
    #[error("error building presigned get url: {0}")]
    GetPresignedUrl(String),
    #[error("error building presigned put url: {0}")]
    PutPresignedUrl(String),
}

pub async fn list_buckets() -> Result<ListBucketsOutput, S3Error> {
    let config = config::Config::new().await;
    let client = Client::new(&config);
    match client.list_buckets().send().await {
        Ok(output) => Ok(output),
        Err(err) => Err(S3Error::ListBuckets(err.to_string())),
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
            Err(err) => Err(S3Error::GetObject(err.to_string())),
        }
    }

    pub async fn put_object(&self, key: &str, body: Vec<u8>) -> Result<PutObjectOutput, S3Error> {
        let Self { client, name } = self;
        let request = client.put_object().bucket(name).body(body.into()).key(key);
        match request.send().await {
            Ok(output) => Ok(output),
            Err(err) => Err(S3Error::PutObject(err.to_string())),
        }
    }

    pub async fn delete_object(&self, key: &str) -> Result<DeleteObjectOutput, S3Error> {
        let Self { client, name } = self;
        let request = client.delete_object().bucket(name).key(key);
        match request.send().await {
            Ok(output) => Ok(output),
            Err(err) => Err(S3Error::DeleteObject(err.to_string())),
        }
    }

    fn _build_presigned_config(duration: Duration) -> Result<PresigningConfig, S3Error> {
        match PresigningConfig::expires_in(duration) {
            Ok(config) => Ok(config),
            Err(err) => Err(S3Error::PresignedConfig(err.to_string())),
        }
    }

    pub async fn get_presigned_url(
        &self,
        key: &str,
        expires_in: Duration,
    ) -> Result<String, S3Error> {
        let Self { client, name } = self;
        let presigned = Self::_build_presigned_config(expires_in)?;
        match client
            .get_object()
            .bucket(name)
            .key(key)
            .presigned(presigned)
            .await
        {
            Ok(a) => Ok(a.uri().to_string()),
            Err(err) => Err(S3Error::GetPresignedUrl(err.to_string())),
        }
    }

    pub async fn put_presigned_url(
        &self,
        key: &str,
        expires_in: Duration,
    ) -> Result<String, S3Error> {
        let Self { client, name } = self;
        let presigned = Self::_build_presigned_config(expires_in)?;
        match client
            .put_object()
            .bucket(name)
            .key(key)
            .presigned(presigned)
            .await
        {
            Ok(response) => Ok(response.uri().to_string()),
            Err(err) => Err(S3Error::PutPresignedUrl(err.to_string())),
        }
    }
}
