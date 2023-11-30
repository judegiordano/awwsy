use aws_config::{BehaviorVersion, SdkConfig};
use once_cell::sync::Lazy;
use std::sync::Arc;

pub static CONFIG: Lazy<Arc<SdkConfig>> = Lazy::new(|| {
    futures::executor::block_on(async {
        Arc::new(aws_config::defaults(BehaviorVersion::latest()).load().await)
    })
});
