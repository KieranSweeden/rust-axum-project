#![allow(unused)]

use std::net::SocketAddr;

use axum::{
    response::{Html, IntoResponse},
    routing::get,
    Router,
};

#[tokio::main]
async fn main() {
    let routes_hello = Router::new().route("/hello", get(handler_hello));

    // region: --- Start Server
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();

    println!("->> LISTENING on {}\n", listener.local_addr().unwrap());
    axum::serve(listener, routes_hello).await.unwrap();

    // endregion: --- Start Server
}

// region: -- Handler Hello
async fn handler_hello() -> impl IntoResponse {
    println!("->> {:<12} - handler_hello", "HANDLER");

    Html("Hello <strong>World!!!</strong>")
}

// endregion: -- Handler Hello
