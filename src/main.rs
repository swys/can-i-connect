// allows us to: `use crate::{Error, Result};`
pub use self::error::{Error, Result};

// modules
mod argc;
mod can_i_connect;
mod error;
mod helpers;
mod integration_tests;
mod options;
mod web;

// imports
use crate::can_i_connect::CanIConnect;
use crate::options::Options;
use argc::argc_app;
use helpers::create_logger;
use log::{error, info};
use reqwest::Client;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<()> {
	// options setup
	let argc = argc_app().get_matches();
	let options = match Options::from_argc(argc) {
		Ok(options) => options,
		Err(e) => panic!("{}", e),
	};
	// logger setup
	create_logger(options.no_color)
		.filter_level(options.log_level)
		.init();

	// can_i setup
	let can_i_connect = CanIConnect {
		http: options.http_hosts,
		tcp: options.tcp_hosts,
		timeout: options.timeout,
		server_mode: !options.listen.is_empty(),
		listen_addr: options.listen,
		http_client: Some(
			Client::builder()
				.timeout(Duration::from_secs(options.timeout as u64))
				.build()
				.unwrap(),
		),
	};

	// figure out if we are running in server mode (via --listen) or CLI mode
	if can_i_connect.server_mode {
		// we are in server mode
		info!(
			"Server mode Activated! Listening on: {}",
			can_i_connect.listen_addr
		);
		can_i_connect.bind().await;
	} else {
		// we are in CLI mode
		let connection_results = can_i_connect.connection_report().await;
		info!(
			"Successfully connected to [{}] hosts out of [{}] total hosts",
			connection_results.successful_hosts.len(),
			can_i_connect.hosts_total(),
		);
		if connection_results.failed_hosts.len() > 0 {
			error!(
				"Failed to connect to the following [{}] host(s): \n[{}]",
				connection_results.failed_hosts.len(),
				connection_results.failed_hosts.join("\n")
			);
		}
	}
	Ok(())
}
