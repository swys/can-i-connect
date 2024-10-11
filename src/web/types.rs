use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;

use crate::options::DEFAULT_TIMEOUT;

// region structs
#[derive(Debug, Deserialize, Serialize)]
pub struct CanIConnectPayload {
	#[serde(default = "default_hosts")]
	pub http_hosts: Vec<String>,
	#[serde(default = "default_hosts")]
	pub tcp_hosts: Vec<String>,
	#[serde(default = "default_timeout", deserialize_with = "deserialize_timeout")]
	pub timeout: usize,
}

fn default_timeout() -> usize {
	DEFAULT_TIMEOUT
}

fn default_hosts() -> Vec<String> {
	vec![]
}

// Custom deserialization function for the timeout field
fn deserialize_timeout<'de, D>(deserializer: D) -> Result<usize, D::Error>
where
	D: Deserializer<'de>,
{
	// Attempt to deserialize the value as a number
	let value: Value = Value::deserialize(deserializer)?;

	// Check the type of the value and convert accordingly
	match value {
		Value::Number(num) => num
			.as_u64()
			.map(|n| n as usize)
			.ok_or_else(|| serde::de::Error::custom("timeout must be a valid number")),
		Value::String(s) => s
			.parse::<usize>()
			.map_err(|_| serde::de::Error::custom("timeout must be a valid number")),
		Value::Null => Ok(default_timeout()), // Return default if the value is null
		_ => Err(serde::de::Error::custom(
			"timeout must be a number or string",
		)),
	}
}

#[derive(Debug)]
pub struct ValidatedHosts {
	pub http_hosts: Vec<String>,
	pub tcp_hosts: Vec<String>,
}
// endregion structs
