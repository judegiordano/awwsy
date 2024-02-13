#[cfg(test)]
mod unit {
	use anyhow::Result;
	// use crate::s3::list_buckets;

	// #[tokio::test]
	// async fn test_one() -> Result<()> {
	// 	// let buckets = list_buckets().await?;
	// 	// println!("{:?}", buckets);
	// 	Ok(())
	// }

	// #[tokio::test]
	// async fn test_two() -> Result<()> {
	// 	// let buckets = list_buckets().await?;
	// 	// println!("{:?}", buckets);
	// 	Ok(())
	// }

	mod polly {
		use anyhow::Result;
		use chrono::{Date, Utc};
		use serde::{Deserialize, Serialize};
		use serde_json::json;
		use tracing_subscriber::FmtSubscriber;

		use crate::dynamodb::Table;

		// use crate::my_macro;

		// use crate::my_macro;

		#[tokio::test]
		async fn test_two() -> Result<()> {
			let subscriber = FmtSubscriber::builder()
				.with_max_level(tracing::Level::DEBUG)
				.finish();
			tracing::subscriber::set_global_default(subscriber)?;

			#[derive(Default, Debug, Deserialize, Serialize, Clone)]
			struct SubData {
				pub value_one: String,
				pub value_two: u32,
				pub test_array: Vec<u32>,
			}

			#[derive(Default, Debug, Deserialize, Serialize, Clone)]
			struct TestTable {
				pub pk: String,
				pub sk: String,
				pub sub_object: SubData,
				pub created_at: chrono::DateTime<Utc>,
				pub updated_at: chrono::DateTime<Utc>,
			}

			impl Table for TestTable {
				fn table_name() -> String {
					"testing-table".to_string()
				}
			}

			let new_item = TestTable {
				pk: nanoid::nanoid!(),
				sk: nanoid::nanoid!(),
				sub_object: SubData {
					value_one: "test".to_string(),
					value_two: 1,
					test_array: [1, 2, 3].to_vec(),
				},
				..Default::default()
			}
			.save()
			.await?;
			println!("INSERTED: {:#?}", new_item);
			// let result = TestTable::get_by_pk(
			// 	json!({
			// 		"pk": "clKERRuXrb57SNVmG-PP1",
			// 		"sk": "QATFH346WO-n-RYikHIpR"
			// 	}),
			// 	None,
			// )
			// .await?;
			// println!("{:#?}", result);

			// rust-temp-single-table-collection
			// #[derive(Default, Debug, Deserialize, Serialize, Clone)]
			// struct MyTable {
			// 	pk: String,
			// 	sk: String,
			// 	username: String,
			// 	user_id: String,
			// 	entity: String,
			// 	gs1_hash: String,
			// 	gs1_sort: String,
			// }
			// impl Table for MyTable {
			// 	fn table_name() -> String {
			// 		"rust-temp-single-table-collection".to_string()
			// 	}
			// }
			// let result = MyTable::query(
			// 	json!({
			// 		// "gs1_hash": "user#",
			// 		// "gs1_sort": "server#cl6s2xxh90000uioo4ukertin#username#some_username_1974"
			// 	}),
			// 	"gs1".to_string(),
			// )
			// .await;
			// let result = MyTable::query(
			// 	json!({
			// 		"gs1_hash": "user#",
			// 		"gs1_sort": "server#cl6s2xxh90000uioo4ukertin#username#some_username_1974"
			// 	}),
			// 	"gs1".to_string(),
			// )
			// .await;
			// let result = MyTable::query_by_pk(json!({
			// 	"pk": "user#cl6uunpg40001ui44ijx11a8e",
			// 	"sk": "user#"
			// }))
			// .await;
			// let result = MyTable::get_by_pk(
			// 	json!({
			// 		"pk": "user#cl6uunpg40001ui44ijx11a8e",
			// 		"sk": "user#"
			// 	}),
			// 	None, // Some("username".to_string()),
			// )
			// .await;
			// println!("{:#?}", result);

			// #[derive(Debug, Deserialize, Serialize, Default, Clone)]
			// struct UserSettings {
			// 	active: bool,
			// 	name: String,
			// 	setting_name: Option<String>,
			// 	tweaks: Vec<String>,
			// }

			// #[derive(Default, Debug, Deserialize, Serialize, Clone)]
			// struct User {
			// 	username: String,
			// 	activated: bool,
			// 	age: u32,
			// 	nickname: Option<String>,
			// 	settings: UserSettings,
			// 	long_lat: Vec<u32>,
			// 	updated_at: String,
			// }
			// let user = User {
			// 	username: "my_username".to_string(),
			// 	activated: true,
			// 	age: 26,
			// 	nickname: None,
			// 	long_lat: vec![1, 2, 3],
			// 	settings: UserSettings {
			// 		active: false,
			// 		name: "my config".to_string(),
			// 		tweaks: vec!["tweak_one".to_string(), "tweak_two".to_string()],
			// 		setting_name: Some("testing".to_string()),
			// 	},
			// 	..Default::default()
			// };
			// impl Table for User {}
			// User::query_one(json!({ "pk": "user#", "sk": 23 })).await;
			// let inserted = user.put().await?;
			// println!("{:?}", inserted);
			// let table = Table::new();
			// table.put_item(user).await?;
			Ok(())
		}
	}
}
