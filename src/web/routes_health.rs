use crate::helpers::handler_log;
use crate::version::VERSION;
use axum::{extract::OriginalUri, response::IntoResponse, routing::get, Json, Router};
use log::debug;
use reqwest::StatusCode;
use serde_json::json;

pub fn routes() -> Router {
	Router::new().route("/health", get(health_handler))
}

async fn health_handler(
	OriginalUri(original_uri): OriginalUri,
) -> Result<impl IntoResponse, StatusCode> {
	let path = original_uri.path();
	debug!("{}", handler_log(path));
	Ok((
		StatusCode::OK,
		Json(json!({
			"healthy": true,
			"version": *VERSION,
		})),
	))
}
