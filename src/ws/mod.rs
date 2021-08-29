use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use tokio::net::TcpStream;
use tokio_tungstenite::tungstenite::{
	protocol::{frame::coding::CloseCode, CloseFrame},
	Message,
};

pub fn validate(_message: String) {}

#[derive(Serialize, Deserialize, Debug)]
pub struct WebSocketMessage {
	pub t: u8,
	pub e: Option<u8>,
	pub d: Option<WebSocketMessageData>,
	pub n: usize,
	pub s: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WebSocketSessionGrant {
	session_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum WebSocketMessageData {
	SessionGrant(WebSocketSessionGrant),
}

pub async unsafe fn accept_connection(stream: TcpStream) {
	let addr = stream
		.peer_addr()
		.expect("connected streams should have a peer address");
	println!("Peer address: {}", addr);

	let ws_stream = tokio_tungstenite::accept_async(stream)
		.await
		.expect("Error during the websocket handshake occurred");

	println!("New WebSocket connection: {}", addr);

	let (mut write, _read) = ws_stream.split();

	let session_grant_json = &WebSocketMessage {
		t: 0,
		e: None,
		d: Some(WebSocketMessageData::SessionGrant(WebSocketSessionGrant {
			session_id: "uwu".to_string(),
		})),
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

		println!("Sent session grant: {:#?}", session_grant_json);
	}
}
