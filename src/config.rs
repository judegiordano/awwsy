use async_once::AsyncOnce;
use aws_config::{BehaviorVersion, SdkConfig};
use lazy_static::lazy_static;

lazy_static! {
	pub static ref CONFIG: AsyncOnce<SdkConfig> =
		AsyncOnce::new(async { aws_config::defaults(BehaviorVersion::latest()).load().await });
}
