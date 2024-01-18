use aws_config::{BehaviorVersion, SdkConfig};
use once_cell::sync::Lazy;

pub static CONFIG: Lazy<SdkConfig> = Lazy::new(|| {
	futures::executor::block_on(async {
		aws_config::defaults(BehaviorVersion::latest()).load().await
	})
});
