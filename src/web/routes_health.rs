use crate::helpers::handler_log;
use crate::Result;
use axum::{extract::Request, routing::get, Json, Router};
use clap::crate_version;
use log::debug;
use serde_json::{json, Value};

pub fn routes() -> Router {
	Router::new().route("/health", get(health_handler))
}

async fn health_handler(req: Request) -> Result<Json<Value>> {
	let path = req.uri().path();
	debug!("{}", handler_log(path));
	Ok(Json(json!({
		"healthy": true,
		"version": crate_version!(),
	})))
}
