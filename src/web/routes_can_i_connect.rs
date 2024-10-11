use crate::{
	helpers::{handler_log, was_successful},
	web::route_helpers::{parse_payload_without_timeout, validate_hosts},
	CanIConnect,
};
use axum::{
	extract::OriginalUri, http::StatusCode, response::IntoResponse, routing::post, Json, Router,
};
use log::debug;
use reqwest::Client;
use serde_json::{json, Value};
use std::time::Duration;

pub fn routes() -> Router {
	Router::new().route("/can-i-connect", post(can_i_connect_handler))
}

pub async fn can_i_connect_handler(
	OriginalUri(original_uri): OriginalUri,
	Json(mut raw_payload): Json<Value>,
) -> impl IntoResponse {
	let path = original_uri.path();
	debug!("{}", handler_log(path));
	debug!("{:?}", raw_payload);

	// Parse the rest of the payload without the timeout field
	let payload = match parse_payload_without_timeout(&mut raw_payload) {
		Ok(p) => p,
		Err(err) => return Err(err),
	};

	debug!("timeout: {}", payload.timeout);
	debug!("data: {:?}", payload);

	// Validate hosts
	validate_hosts(&payload)?;

	// Try to build the HTTP client and handle errors
	let http_client = match Client::builder()
		.timeout(Duration::from_secs(payload.timeout as u64))
		.build()
	{
		Ok(client) => Some(client),
		Err(_) => {
			let error_body = Json(json!({
					"error": "Failed to create HTTP client."
			}));
			return Err((StatusCode::INTERNAL_SERVER_ERROR, error_body));
		}
	};

	// can_i setup
	let can_i_connect = CanIConnect {
		http: payload.http_hosts,
		tcp: payload.tcp_hosts,
		timeout: payload.timeout,
		server_mode: false,
		listen_addr: String::from(""),
		http_client,
	};
	debug!("{:#?}", can_i_connect);
	// check connectivity and report results
	let connection_results = can_i_connect.connection_report().await;

	// Create the success body.
	let resp_payload = Json(json!({
		"success": was_successful(connection_results.failed_hosts.clone()),
		"connection_report": {
			"failures": {
				"hosts_unreachable": &connection_results.failed_hosts.len(),
				"failed_hosts_list": &connection_results.failed_hosts,
			},
			"successful": {
				"hosts_reachable": &connection_results.successful_hosts.len(),
				"successful_hosts_list": &connection_results.successful_hosts,
			},
		},
	}));
	Ok((StatusCode::OK, resp_payload))
}
