#![allow(unused)]

use anyhow::Result;
use serde_json::json;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let http_client = httpc_test::new_client("http://127.0.0.1:8080")?;

    http_client.do_get("/hello2/Mike").await?.print().await?;

    let req_login = http_client.do_post(
        "/api/login",
        json!({
            "username": "demo1",
            "pwd": "welcome"
        }),
    );

    req_login.await?.print().await?;

    Ok(())
}
