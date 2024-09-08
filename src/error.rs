use clap::Error as clap_error;
use derive_more::{Display, From};
use reqwest::Error as req_err;
use std::error::Error as StdError;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, From, Display)]
pub enum Error {
	// -- Internals
	#[display("--log-level must be a one of [info|error|debug] but got {}", _0)]
	InvalidLogLevel(String),
	#[display("--timeout must be a number but got {}", _0)]
	InvalidTimeout(String),
	#[display("request took longer than {} seconds", _0)]
	RequestTimedOut(usize),
	#[display("No hosts supplied. Must supply hosts through --http-hosts or --tcp-hosts args. Both cannot be empty!")]
	NoHostsSupplied,
	DNSResolutionFailed(String),

	// -- Externals
	#[from]
	Clap(clap_error),
	#[from]
	ReqwestError(req_err),
}

impl StdError for Error {}
