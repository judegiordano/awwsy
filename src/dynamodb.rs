use crate::{config::CONFIG, types::AwwsyError};
use aws_sdk_dynamodb::{
	error::{ProvideErrorMetadata, SdkError},
	operation::{
		batch_write_item::BatchWriteItemOutput, get_item::GetItemOutput, put_item::PutItemOutput,
		query::QueryOutput,
	},
	types::{AttributeValue, PutRequest, Select, WriteRequest},
	Client,
};
use convert_case::{Case, Casing};
use serde::{de::DeserializeOwned, Serialize};
use serde_json::{json, Map, Value};
use std::{collections::HashMap, fmt::Debug, ops::Deref};

fn map_sdk_error<E: ProvideErrorMetadata + Debug, R: Debug>(err: SdkError<E, R>) -> AwwsyError {
	tracing::error!("[AWS SDK ERROR]: {:?}", err);
	let message = err.message().map_or(err.to_string(), |msg| msg.to_string());
	AwwsyError::PollyError(message)
}

fn map_error(err: impl std::error::Error) -> AwwsyError {
	tracing::error!("[AWWSY ERROR]: {:?}", err);
	AwwsyError::PollyError(err.to_string())
}

#[allow(async_fn_in_trait)]
pub trait Table
where
	Self: DeserializeOwned + Serialize + Default + Clone,
{
	fn table_name() -> String {
		let name = std::any::type_name::<Self>();
		name.split("::").last().map_or_else(
			|| name.to_string(),
			|name| {
				let mut normalized = name.to_case(Case::Snake);
				if !normalized.ends_with('s') {
					normalized.push('s');
				}
				normalized
			},
		)
	}

	fn new() -> Client {
		Client::new(&CONFIG)
	}

	fn _attribute_to_value(attribute: &AttributeValue) -> Value {
		match attribute {
			AttributeValue::Bool(value) => Value::Bool(*value),
			// AttributeValue::B(_) => todo!(),
			// AttributeValue::Bs(_) => todo!(),
			// AttributeValue::L(_) => todo!(),
			// AttributeValue::M(_) => todo!(),
			AttributeValue::N(value) => Value::Number(value.parse().unwrap()),
			// AttributeValue::Ns(_) => todo!(),
			// AttributeValue::Null(_) => todo!(),
			AttributeValue::S(value) => Value::String(value.to_string()),
			// AttributeValue::Ss(_) => todo!(),
			// AttributeValue::Unknown => todo!(),
			_ => todo!(),
		}
	}

	fn _convert_to_struct(object: &HashMap<String, AttributeValue>) -> Result<Self, AwwsyError> {
		let mut json = Map::new();
		object.iter().for_each(|(key, value)| {
			let value = match value {
				// AttributeValue::B(value) => Value::String(()),
				// AttributeValue::Bs(value) => Value::String(()),
				AttributeValue::Bool(value) => Value::Bool(value.to_owned()),
				// AttributeValue::L(value) => Value::String(()),
				AttributeValue::M(value) => {
					let object = value.iter().fold(Map::new(), |mut map, (key, value)| {
						map.insert(key.to_string(), Self::_attribute_to_value(value));
						map
					});
					Value::Object(object)
				}
				AttributeValue::N(value) => Value::Number(value.parse().unwrap()),
				AttributeValue::Ns(value) => Value::Array(
					value
						.iter()
						.map(|a| Value::Number(a.parse().unwrap()))
						.collect(),
				),
				AttributeValue::Null(_) => Value::Null,
				AttributeValue::S(value) => Value::String(value.to_string()),
				AttributeValue::Ss(value) => {
					value.iter().map(|a| Value::String(a.to_string())).collect()
				}
				_ => Value::Null,
			};
			json.insert(key.to_string(), value);
		});
		serde_json::from_value(Value::Object(json)).map_err(map_error)
	}

	fn _flatten_array(values: &Vec<Value>) -> AttributeValue {
		let mut numbers = vec![];
		let mut strings = vec![];
		values.iter().for_each(|value| match value {
			Value::Number(value) => numbers.push(value.to_string()),
			Value::String(value) => strings.push(value.to_string()),
			_ => todo!(),
		});
		if numbers.len() > strings.len() {
			return AttributeValue::Ns(numbers);
		}
		AttributeValue::Ss(strings)
	}

	fn _flatten_object(object: Map<String, Value>) -> HashMap<String, AttributeValue> {
		object
			.iter()
			.fold(HashMap::new(), |mut hashmap, (key, value)| {
				hashmap.insert(key.to_string(), Self::_value_to_attribute_value(value));
				hashmap
			})
	}

	fn _convert_to_attribute_map(&self) -> Result<HashMap<String, AttributeValue>, AwwsyError> {
		let json = serde_json::to_value(self).map_err(map_error)?;
		let object = match json.as_object() {
			Some(object) => object.to_owned(),
			None => {
				return Err(AwwsyError::DynamoDbError(
					"no object found to unwrap".to_string(),
				))
			}
		};
		Ok(Self::_flatten_object(object))
	}

	fn _value_to_attribute_value(value: &Value) -> AttributeValue {
		match value {
			Value::Null => AttributeValue::Null(true),
			Value::Bool(value) => AttributeValue::Bool(*value),
			Value::Number(value) => AttributeValue::N(value.to_string()),
			Value::String(value) => AttributeValue::S(value.to_string()),
			Value::Object(value) => AttributeValue::M(Self::_flatten_object(value.to_owned())),
			Value::Array(value) => Self::_flatten_array(&value),
		}
	}

	async fn save(&self) -> Result<Self, AwwsyError> {
		let mut item = self._convert_to_attribute_map()?;
		let now = chrono::Utc::now();
		item.insert("updated_at".to_string(), AttributeValue::S(now.to_string()));
		item.insert("created_at".to_string(), AttributeValue::S(now.to_string()));
		Self::new()
			.put_item()
			.set_item(Some(item))
			.table_name(Self::table_name())
			.send()
			.await
			.map_err(map_sdk_error)?;
		Ok(self.clone())
	}

	/// ```rust
	///
	///	let result = MyTable::get_by_pk(
	///		json!({
	///			"pk": "user#",
	///			"sk": "server#cl6s2xxh90000uioo4ukertin#username#some_username_1974"
	///		}),
	/// 	Some("Description, RelatedItems[0], ProductReviews.FiveStar".to_string()),
	///	)
	/// .await;
	/// ```
	async fn get_by_pk(query: Value, select: Option<String>) -> Result<Self, AwwsyError> {
		let keys =
			query
				.as_object()
				.unwrap()
				.iter()
				.fold(HashMap::new(), |mut keys, (key, value)| {
					keys.insert(key.to_string(), Self::_value_to_attribute_value(value));
					keys
				});
		let item = Self::new()
			.get_item()
			.table_name(Self::table_name())
			.set_key(Some(keys))
			.set_projection_expression(select)
			.send()
			.await
			.map_err(map_sdk_error)?;
		let item = item.item().unwrap();
		Self::_convert_to_struct(item)
	}

	/// ```rust
	///
	///	let result = MyTable::query(
	///		json!({
	///			"gs1_hash": "user#",
	///			"gs1_sort": "server#cl6s2xxh90000uioo4ukertin#username#some_username_1974"
	///		}),
	///		"gs1".to_string(),
	///	)
	/// .await;
	/// ```
	async fn query(query: Value, index_name: String) -> Result<QueryOutput, AwwsyError> {
		let (expression_attribute_names, expression_attribute_values, filter) =
			query.as_object().unwrap().iter().fold(
				(HashMap::new(), HashMap::new(), Vec::new()),
				|(mut expressions_names, mut expressions_values, mut filters), (key, value)| {
					let expression_name = format!("#{key}");
					let expression_value = format!(":{key}");
					filters.push(format!("{expression_name} = {expression_value}"));
					expressions_names.insert(expression_name, key.to_string());
					expressions_values
						.insert(expression_value, Self::_value_to_attribute_value(value));
					(expressions_names, expressions_values, filters)
				},
			);
		Self::new()
			.query()
			.set_expression_attribute_names(Some(expression_attribute_names))
			.set_expression_attribute_values(Some(expression_attribute_values))
			.set_key_condition_expression(Some(filter.join(" AND ")))
			.set_index_name(Some(index_name.to_string()))
			.select(Select::AllAttributes)
			.table_name(Self::table_name())
			.send()
			.await
			.map_err(map_sdk_error)
	}

	async fn batch_insert(objects: Vec<Self>) -> Result<BatchWriteItemOutput, AwwsyError> {
		let now = chrono::Utc::now();
		let a = objects
			.iter()
			.map(|object| object._convert_to_attribute_map().map_err(map_error))
			.filter_map(Result::ok)
			.collect::<Vec<_>>();
		let requests = objects.iter().fold(vec![], |mut operations, object| {
			let mut item = object._convert_to_attribute_map().unwrap();
			item.insert("updated_at".to_string(), AttributeValue::S(now.to_string()));
			item.insert("created_at".to_string(), AttributeValue::S(now.to_string()));
			let input = PutRequest::builder().set_item(Some(item)).build().unwrap();
			operations.push(WriteRequest::builder().set_put_request(Some(input)).build());
			operations
		});
		let mut operations = HashMap::new();
		operations.insert(Self::table_name(), requests);
		Self::new()
			.batch_write_item()
			.set_request_items(Some(operations))
			.send()
			.await
			.map_err(map_sdk_error)
	}
}
