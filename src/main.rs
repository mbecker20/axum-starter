use std::{net::SocketAddr, str::FromStr};

use anyhow::Context;
use axum::{routing::get, Router};

#[macro_use]
extern crate tracing;

async fn app() -> anyhow::Result<()> {
  let app =
    Router::new().route("/", get(|| async { "hello, world" }));

  let socket_addr = SocketAddr::from_str("0.0.0.0:80")
    .context("failed to parse socket addr")?;

  let listener = tokio::net::TcpListener::bind(&socket_addr)
    .await
    .context("failed to bind to tcp listener")?;

  info!("axum starter listening on {socket_addr}");

  axum::serve(listener, app).await.context("server crashed")?;

  Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  dotenv::dotenv().ok();
  tracing_subscriber::fmt::init();

  let mut term_signal = tokio::signal::unix::signal(
    tokio::signal::unix::SignalKind::terminate(),
  )?;

  let app = tokio::spawn(app());

  tokio::select! {
    res = app => return res?,
    _ = term_signal.recv() => {},
  }

  Ok(())
}
