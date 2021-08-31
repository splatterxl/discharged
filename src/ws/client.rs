use std::net::SocketAddr;

use futures_util::{
	stream::{SplitSink, SplitStream},
	SinkExt, StreamExt,
};
use serde_json::to_string;
use tokio::net::TcpStream;
use tokio_tungstenite::{tungstenite::Message, WebSocketStream};

use crate::ws::schemas::dispatches::Greetings;

use super::{
	errors::DEFAULT_CLOSE_FRAME,
	schemas::{
		dispatches::data_types::PartialUser,
		gen::{dispatch, greetings},
	},
};

pub struct WebSocketClient {
	pub streams: (
		SplitSink<WebSocketStream<TcpStream>, Message>,
		SplitStream<WebSocketStream<TcpStream>>,
	),
	pub session_token: String,
	/// The IP address and port of the client
	pub client_addr: SocketAddr,
}

impl WebSocketClient {
	pub async fn new(
		streams: (
			SplitSink<WebSocketStream<TcpStream>, Message>,
			SplitStream<WebSocketStream<TcpStream>>,
		),
		session_token: String,
		client_addr: SocketAddr,
	) -> String {
		let (mut write, read) = streams;

		if let Err(_) = WebSocketClient::greet(&mut write, &client_addr).await {
			write
				.send(Message::Close(Some(DEFAULT_CLOSE_FRAME)))
				.await
				.unwrap_or(());
			return session_token;
		}

		let client = Self {
			streams: (write, read),
			session_token: session_token.clone(),
			client_addr,
		};

		tokio::spawn(WebSocketClient::handle(client));

		session_token
	}

	pub async fn greet(
		write: &mut SplitSink<WebSocketStream<TcpStream>, Message>,
		addr: &SocketAddr,
	) -> Result<(), ()> {
		let user = PartialUser {
			username: String::from("Splatterxl"),
			nickname: String::from(""),
			id: 0,
		};

		println!("{}: greet {} ({})", addr, &user.username, &user.id);

		let res = write
			.send(Message::Text(
				to_string(&dispatch::<Greetings>(Some(greetings(user)), 1, None, 1)).unwrap_or(String::from(""))
			))
			.await;

        if let Err(_) = res {
            return Err(());
        }

		Ok(())
	}

	pub async fn handle(client: Self) {
		let (_write, mut read) = client.streams;
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
