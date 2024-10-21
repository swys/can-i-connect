use crate::{
	dns::{DefaultResolver, DnsResolver},
	error::{Error, Result},
};
use ansi_term::Colour;
use env_logger::{Builder, Target};
use log::{debug, error, warn, Level, LevelFilter, Record};
use reqwest::Client;
use std::{
	io::Write,
	net::{SocketAddr, SocketAddrV6, TcpStream},
	str::FromStr,
	sync::Arc,
	time::Duration,
};

// region: functions
pub fn parse_log_level(level: &String) -> Result<LevelFilter> {
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

// Returns the first ip address the given host resolves to (prefers IPV4, falls back to IPV6)
pub fn get_address(resolver: &dyn DnsResolver, host: &str) -> Result<Option<SocketAddr>> {
	debug!("Attempting to resolve dns for address: {}", host);
	let addrs = resolver.resolve(host)?;
	let mut ipv6_fallback: Option<SocketAddrV6> = None;
	for addr in addrs {
		match addr {
			SocketAddr::V4(ipv4_addr) => {
				debug!("{} resolved to {} (IPv4)", host, ipv4_addr);
				return Ok(Some(SocketAddr::V4(ipv4_addr)));
			}
			SocketAddr::V6(ipv6_addr) => {
				if ipv6_fallback.is_none() {
					ipv6_fallback = Some(ipv6_addr);
				}
			}
		}
	}
	if let Some(ipv6_addr) = ipv6_fallback {
		debug!(
			"{} did not resolve to any IPv4 addresses, falling back to IPv6: {}",
			host, ipv6_addr
		);
		return Ok(Some(SocketAddr::V6(ipv6_addr)));
	}
	debug!("{} did not resolve to any IPv4 addresses", host);
	Ok(None) // No addresses found
}

pub async fn handle_http(host: &String, client: Option<&Client>, timeout: usize) -> Result<bool> {
	let c = Client::default();
	let client = client.unwrap_or_else(|| &c);
	let resp = client.get(host).send().await;
	match resp {
		Ok(r) => {
			debug!("Request to {} got Response code: {}", host, r.status());
			Ok(true)
		}
		Err(e) => {
			error!("HTTP Error: {}", e);
			if e.is_timeout() {
				Err(Error::RequestTimedOut(timeout))
			} else {
				Err(Error::ReqwestError(Arc::new(e)))
			}
		}
	}
}

pub async fn handle_tcp(host: &String, timeout: usize) -> Result<bool> {
	let timeout = Duration::from_secs(timeout as u64);
	let resolver = DefaultResolver;
	match get_address(&resolver, host) {
		Ok(Some(addr)) => Ok(TcpStream::connect_timeout(&addr, timeout).is_ok()),
		Ok(None) => {
			warn!("Could not resolve DNS for host: {}", host);
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

pub fn validate_bind_addr(addr: &String) -> Result<SocketAddr> {
	match addr.parse::<SocketAddr>() {
		Ok(socket_addr) => Ok(socket_addr),
		Err(_) => Err(Error::InvalidSocketAddr(addr.clone())),
	}
}

pub fn handler_log(path: &str) -> String {
	return format!("->> {:<4} - handler_health - {path}", "HANDLER");
}

pub fn was_successful(failed_hosts: Vec<String>) -> bool {
	if failed_hosts.len() > 0 {
		return false;
	}
	return true;
}

// endregion: functions

// region: unit tests
#[cfg(test)]
pub mod unit_tests {
	use log::LevelFilter;

	use super::{get_address, handler_log, parse_log_level, validate_bind_addr};
	use crate::dns::DnsResolver;
	use crate::error::Error;
	use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4, SocketAddrV6};

	// Setup Mock DNS Resolver
	struct MockResolver {
		addrs: Vec<SocketAddr>,
	}

	impl DnsResolver for MockResolver {
		fn resolve(&self, _host: &str) -> Result<Vec<SocketAddr>, Error> {
			Ok(self.addrs.clone())
		}
	}

	#[test]
	fn get_address_ipv4_test() {
		let resolver = MockResolver {
			addrs: vec![SocketAddr::V4(SocketAddrV4::new(
				Ipv4Addr::new(127, 0, 0, 1),
				8000,
			))],
		};
		let result = get_address(&resolver, "localhost").unwrap();
		assert_eq!(
			result,
			Some(SocketAddr::V4(SocketAddrV4::new(
				Ipv4Addr::new(127, 0, 0, 1),
				8000
			)))
		);
	}

	#[test]
	fn get_address_ipv6_test() {
		let resolver = MockResolver {
			addrs: vec![SocketAddr::V6(SocketAddrV6::new(
				"::1".parse().unwrap(),
				8000,
				0,
				0,
			))],
		};
		let result = get_address(&resolver, "localhost").unwrap();
		assert_eq!(
			result,
			Some(SocketAddr::V6(SocketAddrV6::new(
				"::1".parse().unwrap(),
				8000,
				0,
				0
			)))
		)
	}

	#[test]
	fn get_address_no_addresses() {
		let resolver = MockResolver { addrs: vec![] };
		let result = get_address(&resolver, "localhost").unwrap();
		assert_eq!(result, None);
	}

	#[test]
	fn validate_bind_addr_test() {
		let valid_addr = String::from("127.0.0.1:8000");
		let invalid_addr = String::from("[:::1]:8000");
		match validate_bind_addr(&valid_addr) {
			Ok(addr) => assert_eq!(
				addr.to_string(),
				valid_addr.clone(),
				"{} is a valid address and it should match the return but we get [{}] instead",
				valid_addr,
				addr.to_string()
			),
			Err(e) => panic!("{}", e),
		}

		let invalid_result = validate_bind_addr(&invalid_addr);
		assert!(invalid_result.is_err(), "expected error but got OK");
	}
	#[test]
	fn parse_log_level_test() {
		let valid_debug_log_levels = vec![
			String::from("debug"),
			String::from("DEBUG"),
			String::from("dEbUg"),
		];
		for level in valid_debug_log_levels {
			let result = parse_log_level(&level);
			match result {
				Ok(log_level) => assert_eq!(log_level, LevelFilter::Debug),
				Err(e) => panic!("did not expect to get error but got: [{}]", e.to_string()),
			}
		}
		let valid_error_log_levels = vec![
			String::from("error"),
			String::from("ERROR"),
			String::from("eRroR"),
		];
		for level in valid_error_log_levels {
			let result = parse_log_level(&level);
			match result {
				Ok(log_level) => assert_eq!(log_level, LevelFilter::Error),
				Err(e) => panic!("did not expect to get error but got: [{}]", e.to_string()),
			}
		}
		let invalid_log_levels = vec![String::from("critical")];
		for level in invalid_log_levels {
			let result = parse_log_level(&level);
			assert!(result.is_err(), "expected error but got Ok");
		}
	}

	#[test]
	fn handler_log_test() {
		let path = "/health";
		let result = handler_log(path);
		assert_eq!(
			result, "->> HANDLER - handler_health - /health",
			"unexpected result from handler_log function"
		);
	}
}
// endregion: unit tests
