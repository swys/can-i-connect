#[cfg(test)]
pub mod integration {
	use crate::{
		can_i_connect::{CanIConnect, ConnectionType},
		error::Error,
		web::routes_can_i_connect::can_i_connect_handler,
	};
	use axum::{extract::OriginalUri, http::Uri, response::IntoResponse, Json};
	use http_body_util::BodyExt;
	use httpmock::prelude::*;
	use reqwest::{Client, StatusCode};
	use serde_json::{json, Value};
	use std::time::Duration;
	use tokio;

	// region: Functions
	fn create_server() -> MockServer {
		MockServer::start()
	}

	// endregion: Functions

	// region: Happy Path HTTP hosts
	#[tokio::test]
	async fn happy_path_can_connect_test() {
		let server = create_server();
		server.mock(|when, then| {
			when.path("/hello");
			then.status(200);
		});
		let can_connect = CanIConnect {
			http: vec![server.url("/hello"), server.url("/nonexistent")],
			tcp: vec![server.address().to_string()],
			timeout: 5,
			http_client: None,
			server_mode: false,
			listen_addr: String::from(""),
		};
		for url in &can_connect.http {
			match can_connect.can_connect(ConnectionType::HTTP, url).await {
				Ok(success) => assert!(success),
				Err(e) => {
					panic!("expected success but got error: {}", &e);
				}
			}
		}
	}

	// endregion: Happy Path HTTP hosts

	// region: Unhappy Path HTTP hosts: Connection timeout
	#[tokio::test]
	async fn connection_timeout_can_connect_test() {
		let server = create_server();
		let timeout: usize = 5;
		server.mock(|when, then| {
			when.path("/timeout");
			then.status(200).delay(Duration::from_secs(timeout as u64));
		});
		let can_connect = CanIConnect {
			http: vec![server.url("/timeout")],
			tcp: vec![server.address().to_string()],
			timeout,
			server_mode: false,
			listen_addr: String::from(""),
			http_client: Some(
				Client::builder()
					.timeout(Duration::from_secs(1))
					.build()
					.unwrap(),
			),
		};
		for url in &can_connect.http {
			match can_connect.can_connect(ConnectionType::HTTP, url).await {
				Ok(success) => assert!(
					!success,
					"expecting time out error but got success: {}",
					success
				),
				Err(e) => {
					println!("{e:?}");
					match &e {
						Error::RequestTimedOut(_timeout) => {
							// Test passes since the expected variant is received
						}
						_error => {
							panic!("Expected Error::RequestTimedOut, but got {e:?}");
						}
					}
				}
			}
		}
	}
	// endregion: Unhappy Path HTTP hosts: Connection timeout

	// region: can-i-connect POST with timeout arg
	#[tokio::test]
	async fn can_i_connect_with_timeout_test() {
		let server = create_server();
		server.mock(|when, then| {
			when.path("/hello");
			then.status(200);
		});
		// prepare POST payload with timeout key
		let payolad_with_timeout = Json(json!({
			"http_hosts": [
				format!("{}", server.url("/hello")),
			],
			"tcp_hosts": [
				format!("{}", server.address().to_string()),
			],
			"timeout": 5
		}));
		let uri = Uri::from_static("/can-i-connect");
		let response = can_i_connect_handler(OriginalUri(uri), payolad_with_timeout)
			.await
			.into_response();

		assert_eq!(response.status(), StatusCode::OK);

		// Read the response body
		let body_bytes = response.into_body().collect().await.unwrap().to_bytes();
		let body_json: Value = serde_json::from_slice(&body_bytes).unwrap();
		// Check the expected body content
		let expected_body = json!({
			"connection_report": {
					"failures": {
							"failed_hosts_list": [],
							"hosts_unreachable": 0
					},
					"successful": {
							"hosts_reachable": 2,
							"successful_hosts_list": [
									format!("{}", server.url("/hello")),
									format!("{}", server.address().to_string()),
							]
					}
			},
			"success": true
		});

		assert_eq!(body_json, expected_body);
	}
	// endregion: can-i-connect POST with timeout arg
}
