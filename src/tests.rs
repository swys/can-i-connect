#[cfg(test)]
pub mod unit {
	use crate::{
		error::Error,
		types::{CanIConnect, ConnectionType},
	};
	use httpmock::prelude::*;
	use reqwest::Client;
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
}
