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

    let req_create_ticket = http_client.do_post(
        "/api/tickets",
        json!({
            "title": "Ticket AAA"
        }),
    );

    req_create_ticket.await?.print().await?;

    http_client.do_get("/api/tickets").await?.print().await?;

    Ok(())
}
