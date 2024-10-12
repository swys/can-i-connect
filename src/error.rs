use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use clap::Error as clap_error;
use derive_more::{Display, From};
use reqwest::Error as req_err;
use std::error::Error as StdError;
use std::sync::Arc;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, From, Display, Clone)]
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
	#[display(
		"{} is not a valid bind address, use format <interface>:<port> e.g. 127.0.0.1:8000",
		_0
	)]
	InvalidSocketAddr(String),

	// -- Externals
	#[from]
	Clap(Arc<clap_error>),
	#[from]
	ReqwestError(Arc<req_err>),
}

impl StdError for Error {}

impl IntoResponse for Error {
	fn into_response(self) -> Response {
		println!("->> {:<4} - {self:?}", "INTO_RES");
		// placeholder for axum response
		let mut resp = StatusCode::INTERNAL_SERVER_ERROR.into_response();
		// Insert the Error into the response
		resp.extensions_mut().insert(self);
		resp
	}
}
