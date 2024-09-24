use crate::error::Result;
use crate::helpers::{handle_http, handle_tcp};
use crate::web;
use axum::Router;
use log::{debug, error, info};
use reqwest::Client;
use std::net::SocketAddr;

// region: enums
pub enum ConnectionType {
	HTTP,
	TCP,
}

// endregion: enums

// region: structs
#[derive(Debug, Clone)]
pub struct CanIConnect {
	pub http: Vec<String>,
	pub tcp: Vec<String>,
	pub timeout: usize,
	pub server_mode: bool,
	pub listen_addr: String,
	pub http_client: Option<Client>,
}

#[derive(Debug, Clone)]
pub struct ConnectionReport {
	pub successful_hosts: Vec<String>,
	pub failed_hosts: Vec<String>,
}

// endregion: structs

// region: methods
impl CanIConnect {
	pub async fn can_connect(
		self: &Self,
		connection_type: ConnectionType,
		host: &String,
	) -> Result<bool> {
		let http_clinet_ref = self.http_client.as_ref();
		match connection_type {
			ConnectionType::HTTP => {
				return handle_http(host, http_clinet_ref, self.timeout).await;
			}
			ConnectionType::TCP => {
				return handle_tcp(host, self.timeout).await;
			}
		}
	}
	pub async fn connection_report(self: &Self) -> ConnectionReport {
		let mut result = ConnectionReport {
			successful_hosts: vec![],
			failed_hosts: vec![],
		};
		// check if http hosts are reachable
		for url in self.http.clone() {
			match self.can_connect(ConnectionType::HTTP, &url).await {
				Ok(_) => {
					result.successful_hosts.push(url.to_string());
					info!("successfully connected to {}", url.to_string());
				}
				Err(e) => {
					result.failed_hosts.push(url.to_string());
					error!("{}", e);
				}
			}
		}
		// check if tcp hosts are reachable
		for host in self.tcp.clone() {
			match self.can_connect(ConnectionType::TCP, &host).await {
				Ok(false) => {
					result.failed_hosts.push(host.to_string());
					error!("failed to connect to {}", host.to_string());
				}
				Ok(true) => {
					result.successful_hosts.push(host.to_string());
					info!("successfully connected to {}", host.to_string());
				}
				Err(e) => {
					result.failed_hosts.push(host.to_string());
					error!("{}", e);
				}
			}
		}
		result
	}
	// return total hosts to check
	pub fn hosts_total(self: &Self) -> usize {
		self.http.len() + self.tcp.len()
	}
	// bind to SocketAddr (http server mode)
	pub async fn bind(self: &Self) {
		info!("In Server Mode, listening on: {}", self.listen_addr);
		// handler func
		// setup routes
		let routes_all = Router::new().merge(web::routes_health::routes());
		// start server
		let addr = self.listen_addr.parse::<SocketAddr>().unwrap();
		debug!("Binding to: {addr}");
		let listener = tokio::net::TcpListener::bind(&self.listen_addr)
			.await
			.unwrap();
		axum::serve(listener, routes_all.into_make_service())
			.await
			.unwrap();
	}
}

// endregion: methods

// region: unit tests
#[cfg(test)]
pub mod unit_tests {
	use crate::error::Result;
	#[test]
	fn hello_test() -> Result<()> {
		Ok(())
	}
}
