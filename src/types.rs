use crate::error::{Error, Result};
use ansi_term::Colour;
use clap::ArgMatches;
use env_logger::{Builder, Target};
use log::{debug, error, info, warn, Level, LevelFilter, Record};
use reqwest::Client;
use std::{
	io::Write,
	net::{SocketAddr, SocketAddrV4, TcpStream, ToSocketAddrs},
	str::FromStr,
	time::Duration,
};

// region: constants
const DEFAULT_TIMEOUT: usize = 5;
const DEFAULT_LOG_LEVEL: &str = "info";

// region: functions
fn parse_log_level(level: &String) -> Result<LevelFilter> {
	LevelFilter::from_str(level).map_err(|_| Error::InvalidLogLevel(level.to_string()))
}

pub fn create_logger(with_color: bool) -> Builder {
	let mut builder = Builder::new();
	builder.target(Target::Stdout);

	builder.format(move |buf, record: &Record| {
		let result = if with_color {
			let level = match record.level() {
				Level::Error => Colour::Red.paint("ERROR"),
				Level::Warn => Colour::Yellow.paint("WARN"),
				Level::Info => Colour::Green.paint("INFO"),
				Level::Debug => Colour::Blue.paint("DEBUG"),
				Level::Trace => Colour::Purple.paint("TRACE"),
			};

			writeln!(
				buf,
				"{} [{}:{}] - {}",
				level,
				record.file().unwrap_or("unknown"),
				record.line().unwrap_or(0),
				record.args()
			)
		} else {
			writeln!(
				buf,
				"{} [{}:{}] - {}",
				record.level(),
				record.file().unwrap_or("unknown"),
				record.line().unwrap_or(0),
				record.args()
			)
		};
		result.map_err(|e| e.into())
	});

	builder // Return the owned Builder instance
}

fn get_ipv4_address(host: &str) -> Result<Option<SocketAddrV4>> {
	debug!("Attempting to resolve dns for address: {}", host);
	let addrs = host
		.to_socket_addrs()
		.map_err(|_| Error::DNSResolutionFailed(host.to_string()))?;
	for addr in addrs {
		if let SocketAddr::V4(ipv4_addr) = addr {
			debug!("{} resolved to {} IPv4 address", host, ipv4_addr);
			return Ok(Some(ipv4_addr));
		}
	}
	debug!("{} did not resolve to any IPv4 addresses", host);
	Ok(None) // No IPv4 address found
}

async fn handle_http(host: &String, client: Option<&Client>, timeout: usize) -> Result<bool> {
	let c = Client::default();
	let client = client.unwrap_or_else(|| &c);
	let resp = client.get(host).send().await;
	match resp {
		Ok(r) => {
			debug!("Request to {} got Response code: {}", host, r.status());
			Ok(true)
		}
		Err(e) => {
			if e.is_timeout() {
				Err(Error::RequestTimedOut(timeout))
			} else {
				Err(Error::ReqwestError(e))
			}
		}
	}
}

async fn handle_tcp(host: &String, timeout: usize) -> Result<bool> {
	let timeout = Duration::from_secs(timeout as u64);
	match get_ipv4_address(host) {
		Ok(Some(addr)) => Ok(TcpStream::connect_timeout(&SocketAddr::V4(addr), timeout).is_ok()),
		Ok(None) => {
			warn!("No IPv4 addresses found for host: {}", host);
			Ok(false)
		}
		Err(e) => {
			error!(
				"Error getting IPv4 address for host: {} : {}",
				host,
				e.to_string()
			);
			Ok(false)
		}
	}
}

// endregion: functions

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
	pub http_client: Option<Client>,
}

#[derive(Debug, Clone)]
pub struct ConnectionReport {
	pub successful_hosts: Vec<String>,
	pub failed_hosts: Vec<String>,
}

#[derive(Debug)]
pub struct Options {
	pub http_hosts: Vec<String>,
	pub tcp_hosts: Vec<String>,
	pub timeout: usize,
	pub log_level: LevelFilter,
	pub no_color: bool,
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
	pub fn hosts_total(self: &Self) -> usize {
		self.http.len() + self.tcp.len()
	}
}

impl Options {
	pub fn from_argc(argc: ArgMatches) -> Result<Options> {
		let http_hosts = match argc.get_one::<String>("http-hosts") {
			Some(hosts) => hosts
				.split(",")
				.map(|el| el.to_string())
				.collect::<Vec<String>>(),
			None => {
				debug!("No http hosts supplied");
				vec![]
			}
		};
		let tcp_hosts = match argc.get_one::<String>("tcp-hosts") {
			Some(hosts) => hosts
				.split(",")
				.map(|el| el.to_string())
				.collect::<Vec<String>>(),
			None => {
				debug!("No tcp hosts supplied");
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

		if http_hosts.len() == 0 && tcp_hosts.len() == 0 {
			return Err(Error::NoHostsSupplied);
		}

		Ok(Options {
			http_hosts,
			tcp_hosts,
			timeout,
			log_level,
			no_color,
		})
	}
}

// endregion: methods
