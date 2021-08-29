use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string, Value};
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

pub async unsafe fn accept_connection(stream: TcpStream) {
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
			exchange_interval: 1000 * 60 * 3,
		}),
		n: 0,
		s: None,
	};

	let hello = to_string(hello_json).unwrap_or(String::from("null"));

	if hello == "null" {
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
			.send(Message::Text(hello))
			.await
			.expect("couldn't send HELLO");

		if let Some(Ok(Message::Text(msg))) = read.next().await {
            let msg = from_str::<WebSocketMessage<WebSocketSession>>(&msg.as_str());

            if msg.is_err() {
                write.send(Message::Close(None)).await.expect("couldn't close socket");
                return;
            }

            let msg = msg.unwrap();

            let mut session_token = String::from("");

            &session_token;

            if let Some(data) = msg.d {
                session_token = data.session_token;
            } else {
                write.send(Message::Close(None)).await.expect("couldn't close socket");
                return;
            }
			println!("{}: session token {} received, validating...", &addr, &session_token);

            println!("{}: session token validated", &addr);

			WebSocketClient::new((write, read), session_token, addr);
		}
	}
}
