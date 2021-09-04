#![allow(unused_unsafe)]

use std::{env, error::Error};

use mongodb::{bson::doc, options::ClientOptions, Client};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
	// MongoDB

	let client_options =
		ClientOptions::parse(option_env!("MONGO").expect("MONGO environment variable not defined"))
			.await?;

	let client = Client::with_options(client_options)?;

	client
		.database("admin")
		.run_command(doc! {"ping": 1}, None)
		.await?;

	unsafe { println!("Successfully connect to MongoDB cluster!") }

	// WebSocket TCP Listener
	let addr = env::args().nth(1).unwrap_or(String::from("127.0.0.1:8083"));

	// Create the event loop and TCP listener we'll accept connections on.
	let try_socket = TcpListener::bind(&addr).await;
	let listener = try_socket.expect("Failed to bind");

	unsafe { println!("Listening on: {}", &addr) };

	while let Ok((stream, _)) = listener.accept().await {
		tokio::spawn(discharged::ws::accept_connection(stream));
	}

	Ok(())
}
