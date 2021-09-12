#![allow(unused_unsafe)]

#[macro_use]
extern crate lazy_static;

mod database;
mod types;
mod util;
mod ws;

use std::{env, error::Error};

use database::start as start_database_daemon;
use futures_util::join;
use tokio::{net::TcpListener, spawn};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
	start_database_daemon().await?;

	let ws_task = launch_ws().await;

	Ok(())
}

async fn launch_ws() {
	let addr = env::args().nth(1).unwrap_or(String::from("127.0.0.1:8083"));

	// Create the event loop and TCP listener we'll accept connections on.
	let try_socket = TcpListener::bind(&addr).await;
	let listener = try_socket.expect("Failed to bind");

	unsafe { println!("Listening on: {}", &addr) };

	while let Ok((stream, _)) = listener.accept().await {
		tokio::spawn(ws::accept_connection(stream));
	}
}
