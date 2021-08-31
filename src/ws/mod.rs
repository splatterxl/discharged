use futures_util::{SinkExt, StreamExt};
use serde_json::{from_str, to_string, Value};
use tokio::net::TcpStream;
use tokio_tungstenite::tungstenite::Message;

use crate::ws::{
	client::WebSocketClient,
	errors::DEFAULT_CLOSE_FRAME,
	schemas::{
		dispatches::Hello,
		gen::{dispatch, hello},
		WebSocketMessage, WebSocketSession,
	},
};

pub mod client;
pub mod errors;
pub mod schemas;

pub fn validate(message: String) -> Result<(), ()> {
	match from_str::<WebSocketMessage<Value>>(message.as_str()) {
		Ok(_) => Ok(()),
		Err(_) => Err(()),
	}
}

pub async fn accept_connection(stream: TcpStream) {
	let addr = stream.peer_addr();

	if addr.is_err() {
		println!("Connection request has no Peer Address");
		return;
	}

	let addr = addr.unwrap();

	println!("New connect request (peer address: {})", addr);

	let ws_stream = tokio_tungstenite::accept_async(stream)
		.await
		.expect("Error during the websocket handshake occurred");

	println!("{}: WebSocket connection succeeded", &addr);

	let (mut write, mut read) = ws_stream.split();

	let hello =
		to_string(&dispatch::<Hello>(Some(hello()), 0, None, 0)).unwrap_or(String::from("null"));

	if hello == "null" {
		println!("{}: couldn't stringify hello", addr);
		write
			.send(Message::Close(Some(DEFAULT_CLOSE_FRAME)))
			.await
			.expect("couldn't close");
		return;
	} else {
		write
			.send(Message::Text(hello))
			.await
			.expect("couldn't send HELLO");

		if let Some(Ok(mut msg)) = read.next().await {
			if let Message::Binary(bin) = msg {
				msg = Message::Text(String::from_utf8(bin).unwrap_or(String::from("")));
			}

			match msg {
				Message::Text(mut msg) => {
					msg = String::from(msg.trim());

					let msg = from_str::<WebSocketMessage<WebSocketSession>>(&msg.as_str());

					if msg.is_err() {
						println!("{}: invalid opening packet", addr);
						write
							.send(Message::Close(Some(DEFAULT_CLOSE_FRAME)))
							.await
							.expect("couldn't close socket");
						return;
					}

					let msg = msg.unwrap();

					let mut session_token = String::from("");

					&session_token;

					if let Some(data) = msg.d {
						session_token = data.session_token;
					} else {
						println!("{}: invalid opening packet", addr);
						write
							.send(Message::Close(Some(DEFAULT_CLOSE_FRAME)))
							.await
							.expect("couldn't close socket");
						return;
					}
					println!(
						"{}: session token {} received, validating...",
						&addr, &session_token
					);

					println!("{}: session token validated", &addr);

					WebSocketClient::new((write, read), session_token, addr).await;
				}
				_ => {
					write
						.send(Message::Close(Some(DEFAULT_CLOSE_FRAME)))
						.await
						.unwrap_or(());
				}
			}
		} else {
			write
				.send(Message::Close(Some(DEFAULT_CLOSE_FRAME)))
				.await
				.unwrap_or(());
		}
	}
}
