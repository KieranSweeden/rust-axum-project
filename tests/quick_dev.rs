#![allow(unused)]

use anyhow::Result;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let http_client = httpc_test::new_client("http://127.0.0.1:8080")?;

    http_client.do_get("/hello").await?.print().await?;

    Ok(())
}
