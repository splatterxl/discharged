use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string, Value};
use std::borrow::Cow;
use tokio::net::TcpStream;
use tokio_tungstenite::tungstenite::{
	protocol::{frame::coding::CloseCode, CloseFrame},
	Message,
};

use crate::ws::client::WebSocketClient;

pub mod client;

pub fn validate(message: String) -> Result<(), ()> {
	match from_str::<WebSocketMessage<Value>>(message.as_str()) {
		Ok(_) => Ok(()),
		Err(_) => Err(()),
	}
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WebSocketMessage<T> {
	pub t: u8,
	pub e: Option<u8>,
	pub d: Option<T>,
	pub n: usize,
	pub s: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WebSocketHello {
	exchange_interval: u16,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WebSocketSession {
	session_token: String,
}

pub const DEFAULT_CLOSE_FRAME: CloseFrame = CloseFrame {
	code: CloseCode::Library(1001),
	reason: Cow::Borrowed("Unknown Error"),
};

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

	let hello_json = &WebSocketMessage {
		t: 0,
		e: None,
		d: Some(WebSocketHello {
			exchange_interval: 60106,
		}),
		n: 0,
		s: None,
	};

	let hello = to_string(hello_json).unwrap_or(String::from("null"));

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
            dbg!(&msg);
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

					WebSocketClient::new((write, read), session_token, addr);
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
