use std::{borrow::Cow, net::SocketAddr};

use futures_util::{
	stream::{SplitSink, SplitStream},
	StreamExt,
};
use tokio::net::TcpStream;
use tokio_tungstenite::{
	tungstenite::{
		protocol::{frame::coding::CloseCode, CloseFrame},
		Message,
	},
	WebSocketStream,
};

pub struct WebSocketClient {
	pub streams: (
		SplitSink<WebSocketStream<TcpStream>, Message>,
		SplitStream<WebSocketStream<TcpStream>>,
	),
	pub session_id: String,
	/// The IP address and port of the client
	pub client_addr: SocketAddr,
}

impl WebSocketClient {
	pub fn new(
		streams: (
			SplitSink<WebSocketStream<TcpStream>, Message>,
			SplitStream<WebSocketStream<TcpStream>>,
		),
		session_id: String,
		client_addr: SocketAddr,
	) -> String {
		let client = Self {
			streams,
			session_id: session_id.clone(),
			client_addr,
		};

		tokio::spawn(WebSocketClient::handle(client));

		session_id
	}

	/// basically just drop() but done right :trollface:
	pub async fn handle(client: Self) {
		let (mut write, mut read) = client.streams;
		println!("{}: started listening for messages", &client.client_addr);

		while let Some(Ok(msg)) = read.next().await {
			match msg {
				Message::Close(frame) => {
					println!(
						"{}: connection closed by peer: {:?}",
						client.client_addr, frame
					);
					break;
				}
				_ => todo!(),
			}
		}
	}
}
