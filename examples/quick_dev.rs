use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
	let hc = httpc_test::new_client("http://localhost:8000")?;

	hc.do_get("/health").await?.print().await?;

	Ok(())
}
