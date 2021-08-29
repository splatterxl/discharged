use std::{env, io::Error};

use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Error> {
	let _ = env_logger::try_init();
	let addr = env::args().nth(1).unwrap_or(String::from("127.0.0.1:8083"));

	// Create the event loop and TCP listener we'll accept connections on.
	let try_socket = TcpListener::bind(&addr).await;
	let listener = try_socket.expect("Failed to bind");

	unsafe { println!("Listening on: {}", addr) };

	while let Ok((stream, _)) = listener.accept().await {
		unsafe {
			tokio::spawn(discharged::ws::accept_connection(stream));
		}
	}

	Ok(())
}
