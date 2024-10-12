use crate::error::{Error, Result};
use crate::helpers::{parse_log_level, validate_bind_addr};
use clap::ArgMatches;
use log::LevelFilter;

// region: constants
pub const DEFAULT_TIMEOUT: usize = 5;
const DEFAULT_LOG_LEVEL: &str = "info";

// end region: constants

// region: structs
#[derive(Debug)]
pub struct Options {
	pub http_hosts: Vec<String>,
	pub tcp_hosts: Vec<String>,
	pub timeout: usize,
	pub log_level: LevelFilter,
	pub no_color: bool,
	pub listen: String,
}

// end region: structs

// region: methods
impl Options {
	pub fn from_argc(argc: ArgMatches) -> Result<Options> {
		let http_hosts = match argc.get_one::<String>("http-hosts") {
			Some(hosts) => hosts
				.split(",")
				.map(|el| el.to_string())
				.collect::<Vec<String>>(),
			None => {
				vec![]
			}
		};
		let tcp_hosts = match argc.get_one::<String>("tcp-hosts") {
			Some(hosts) => hosts
				.split(",")
				.map(|el| el.to_string())
				.collect::<Vec<String>>(),
			None => {
				vec![]
			}
		};
		let timeout = match argc.get_one::<String>("timeout") {
			None => DEFAULT_TIMEOUT,
			Some(timeout) => timeout
				.parse::<usize>()
				.map_err(|_| Error::InvalidTimeout(timeout.to_string()))?,
		};
		let level = match argc.get_one::<String>("log-level") {
			Some(level) => String::from(level),
			None => String::from(DEFAULT_LOG_LEVEL),
		};
		let log_level = parse_log_level(&level)?;
		let no_color = argc.get_flag("no-color");

		let listen = match argc.get_one::<String>("listen") {
			Some(bind_addr) => {
				let bind_addr = validate_bind_addr(bind_addr).map_err(|e| e);
				match bind_addr {
					Ok(addr) => addr.to_string(),
					Err(e) => return Err(e),
				}
			}
			None => String::from(""),
		};

		// throw if there are 0 hosts specified and user did not specify to run in server mode via --listen
		if (http_hosts.len() == 0 && tcp_hosts.len() == 0) && listen.is_empty() {
			return Err(Error::NoHostsSupplied);
		}

		Ok(Options {
			http_hosts,
			tcp_hosts,
			timeout,
			log_level,
			no_color,
			listen,
		})
	}
}

// end region: methods
