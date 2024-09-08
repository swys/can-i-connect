// modules
mod argc;
mod error;
mod tests;
mod types;

// imports
use self::error::Result;
use crate::types::{CanIConnect, Options};
use argc::argc_app;
use log::{error, info};
use types::create_logger;

#[tokio::main]
async fn main() -> Result<()> {
	// args setup
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
		http_client: None,
	};
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
	Ok(())
}
