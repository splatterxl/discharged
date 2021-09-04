use std::fmt::Debug;

use futures_util::{SinkExt, StreamExt};
use serde::Deserialize;
use serde_json::{from_str, to_string};
use tokio::net::TcpStream;
use tokio_tungstenite::tungstenite::Message;

use crate::ws::{
	client::WebSocketClient,
	errors::{DEFAULT_CLOSE_FRAME, PARSE_ERROR},
	schemas::{
		dispatches::Hello,
		gen::{dispatch, hello},
		WebSocketMessage, WebSocketSession,
	},
};

use self::schemas::constants::Opcodes;

pub mod client;
pub mod errors;
pub mod schemas;

pub fn validate<'a, T>(message: &'a String) -> Result<(), ()>
where
	T: Deserialize<'a> + Debug,
{
	let obj = from_str::<'a, WebSocketMessage<T>>(message.as_str());
	let opcode = if obj.is_ok() {
		Opcodes::get(obj.as_ref().unwrap().t)
	} else {
		Err(String::new())
	};

	if obj.is_err() || opcode.is_err() {
		unsafe { println!("{:#?} {:#?}", obj, opcode) };
		Err(())
	} else {
		Ok(())
	}
}
pub async fn accept_connection(stream: TcpStream) {
	let addr = stream.peer_addr();

	if addr.is_err() {
		unsafe { println!("Connection request has no Peer Address") };
		return;
	}

	let addr = addr.unwrap();

	unsafe {
		println!("New connect request (peer address: {}) ", addr);
	}

	let ws_stream = tokio_tungstenite::accept_async(stream)
		.await
		.expect("Error during the websocket handshake occurred");

	unsafe { println!("{}: WebSocket connection succeeded", &addr) };

	let (mut write, mut read) = ws_stream.split();

	let hello =
		to_string(&dispatch::<Hello>(Some(hello()), 0, None, 0)).unwrap_or(String::from("null"));

	if hello == "null" {
		unsafe { println!("{}: couldn't stringify hello", addr) };
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

					if validate::<WebSocketSession>(&msg).is_err() {
						unsafe { println!("{}: invalid opening packet", addr) };
						write
							.send(Message::Close(Some(PARSE_ERROR)))
							.await
							.expect("couldn't close socket");
						return;
					}

					let msg =
						from_str::<WebSocketMessage<WebSocketSession>>(&msg.as_str()).unwrap();

					let mut session_token = String::from("");

					let _x = &session_token;
					drop(_x);

					if let Some(data) = msg.d {
						session_token = data.session_token;
					} else {
						unsafe { println!("{}: invalid opening packet", addr) };
						write
							.send(Message::Close(Some(PARSE_ERROR)))
							.await
							.expect("couldn't close socket");
						return;
					}
					unsafe {
						println!(
							"{}: session token \"{}\" received, validating...",
							&addr, &session_token
						);
					}

					unsafe { println!("{}: session token validated", &addr) };

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
