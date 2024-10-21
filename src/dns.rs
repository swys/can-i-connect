use crate::error::Error;
use std::net::{SocketAddr, ToSocketAddrs};

// This is the trait we'll use for DNS resolution
pub trait DnsResolver {
	fn resolve(&self, host: &str) -> Result<Vec<SocketAddr>, Error>;
}

// The default implementation that does the actual DNS resolution
pub struct DefaultResolver;

impl DnsResolver for DefaultResolver {
	fn resolve(&self, host: &str) -> Result<Vec<SocketAddr>, Error> {
		host
			.to_socket_addrs()
			.map(|iter| iter.collect())
			.map_err(|_| Error::DNSResolutionFailed(host.to_string()))
	}
}
