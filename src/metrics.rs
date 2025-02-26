use axum::{
	extract::{MatchedPath, Request},
	middleware::Next,
	response::IntoResponse,
	routing::get,
	Router,
};
use log::debug;
use metrics_exporter_prometheus::{Matcher, PrometheusBuilder, PrometheusHandle};
use std::{future::ready, net::SocketAddr, time::Instant};

// region: constants
const PROMETHEUS_PORT: &str = "9100";
const PROMETHEUS_NAMESPACE: &str = env!("CARGO_PKG_NAME");
// endregion: constants

// region: functions
fn metrics_server() -> Router {
	let recorder_handle = setup_metrics_recorder();
	Router::new().route("/metrics", get(move || ready(recorder_handle.render())))
}

fn setup_metrics_recorder() -> PrometheusHandle {
	const EXPONENTIAL_SECONDS: &[f64] = &[
		0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0, 10.0,
	];
	PrometheusBuilder::new()
		.set_buckets_for_metric(
			Matcher::Full("http_requests_duration_seconds".to_string()),
			EXPONENTIAL_SECONDS,
		)
		.unwrap()
		.install_recorder()
		.unwrap()
}

pub async fn start_metrics_server(addr: String) {
	// parse listen_addr as SocketAddr and panic if it fails
	let socket_addr: SocketAddr = addr
    .parse()
    .expect(
      &format!("Failed to parse socket address '{}'. Please ensure the format is 'IP:PORT' and have valid values", addr)
    );
	let listen_addr = format!("{}:{}", socket_addr.ip(), PROMETHEUS_PORT);
	let server = metrics_server();
	let listener = tokio::net::TcpListener::bind(listen_addr).await.unwrap();
	debug!(
		"metrics server listening on {}",
		listener.local_addr().unwrap()
	);
	axum::serve(listener, server).await.unwrap();
}

pub async fn track_metrics(req: Request, next: Next) -> impl IntoResponse {
	let start = Instant::now();
	let path = if let Some(matched_path) = req.extensions().get::<MatchedPath>() {
		matched_path.as_str().to_owned()
	} else {
		req.uri().path().to_owned()
	};
	let method = req.method().clone();
	let response = next.run(req).await;
	let latency = start.elapsed().as_secs_f64();
	let status = response.status().as_u16().to_string();
	let labels = [
		("method", method.to_string()),
		("path", path.clone()),
		("status", status),
	];
	// define metrics with namespace
	let http_requests_total = format!("{}_http_requests_total", PROMETHEUS_NAMESPACE);
	let http_requests_duration = format!("{}_http_requests_duration_seconds", PROMETHEUS_NAMESPACE);

	metrics::counter!(http_requests_total, &labels).increment(1);
	metrics::histogram!(http_requests_duration, &labels).record(latency);

	response
}

// endregion: functions
