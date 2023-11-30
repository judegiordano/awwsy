use aws_config::{BehaviorVersion, SdkConfig};

pub struct Config {}

impl Config {
    pub async fn new() -> SdkConfig {
        aws_config::defaults(BehaviorVersion::latest()).load().await
    }
}
