use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use tokio::net::TcpStream;
use tokio_tungstenite::tungstenite::{
	protocol::{frame::coding::CloseCode, CloseFrame},
	Message,
};

use crate::ws::client::WebSocketClient;

pub mod client;

pub fn validate(_message: String) {}

#[derive(Serialize, Deserialize, Debug)]
pub struct WebSocketMessage<T> {
	pub t: u8,
	pub e: Option<u8>,
	pub d: Option<T>,
	pub n: usize,
	pub s: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WebSocketSessionGrant {
	session_id: String,
}


pub async unsafe fn accept_connection(stream: TcpStream) {
	let addr = stream
		.peer_addr()
		.expect("connected streams should have a peer address");
	println!("Peer address: {}", addr);

	let ws_stream = tokio_tungstenite::accept_async(stream)
		.await
		.expect("Error during the websocket handshake occurred");

	println!("{}: WebSocket connection succeeded", &addr);

	let (mut write, read) = ws_stream.split();

    let session_id = "uwu".to_string();

	let session_grant_json = &WebSocketMessage {
		t: 0,
		e: None,
		d: Some(WebSocketSessionGrant {
			session_id: session_id.clone()
		}),
		n: 0,
		s: None,
	};

	let session_grant = serde_json::to_string(session_grant_json).unwrap_or(String::from("null"));

	if session_grant == "null" {
		write
			.send(Message::Close(Some(CloseFrame {
				code: CloseCode::Library(1001),
				reason: std::borrow::Cow::Borrowed("Unknown Error"),
			})))
			.await
			.expect("couldn't close");
		return;
	} else {
		write
			.send(Message::Text(session_grant))
			.await
			.expect("couldn't send session grant");

		println!("{}: session {} granted", &addr, &session_id);

        WebSocketClient::new((write, read), session_id, addr);
	}
}
