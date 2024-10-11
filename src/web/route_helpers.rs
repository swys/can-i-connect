use super::types::CanIConnectPayload;
use axum::{extract::Json, http::StatusCode};
use serde_json::{json, Value};
use std::result::Result as StdResult;

// region functions
pub fn parse_payload_without_timeout(
	raw_payload: &mut Value,
) -> Result<CanIConnectPayload, (StatusCode, Json<Value>)> {
	//raw_payload.as_object_mut().unwrap().remove("timeout");

	serde_json::from_value(raw_payload.clone()).map_err(|err| {
		let error_body = Json(json!({
				"error": format!("Failed to deserialize payload: {}", err)
		}));
		(StatusCode::BAD_REQUEST, error_body)
	})
}

pub fn validate_hosts(payload: &CanIConnectPayload) -> StdResult<(), (StatusCode, Json<Value>)> {
	// Unwrap https_hosts and tcp_hosts, default to empty Vec if None
	let http_hosts = payload.http_hosts.clone();
	let tcp_hosts = payload.tcp_hosts.clone();

	if http_hosts.is_empty() && tcp_hosts.is_empty() {
		let error_body = Json(json!({
			"error": "Both 'http_hosts' and 'tcp_hosts' cannot be empty"
		}));
		return Err((StatusCode::BAD_REQUEST, error_body));
	}
	Ok(())
}
