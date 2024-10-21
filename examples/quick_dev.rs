use anyhow::Result;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<()> {
	let hc = httpc_test::new_client("http://localhost:8000")?;

	hc.do_get("/health").await?.print().await?;

	let req_can_i_connect = hc.do_post(
		"/can-i-connect",
		json!({
			"http_hosts": [
				"https://duckduckgo.com",
				"https://rust-lang.org",
			],
			"tcp_hosts": [
				"duckduckgo.com:443",
				"ipv6.google.com:80"
			],
		}),
	);

	req_can_i_connect.await?.print().await?;

	Ok(())
}
