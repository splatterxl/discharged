use std::net::SocketAddr;

use futures_util::{
	stream::{SplitSink, SplitStream},
	SinkExt, StreamExt,
};
use serde_json::to_string;
use tokio::net::TcpStream;
use tokio_tungstenite::{
	tungstenite::{Error, Message},
	WebSocketStream,
};

use crate::{types::ws::GreetingsUser, ws::schemas::dispatches::Greetings};

use super::{
	errors::{DEFAULT_CLOSE_FRAME, PARSE_ERROR},
	schemas::gen::{dispatch, greetings},
};

pub struct WebSocketClient {
	pub write: SplitSink<WebSocketStream<TcpStream>, Message>,
	pub read: SplitStream<WebSocketStream<TcpStream>>,
	/// The authentication token of the session. Account tokens are **not** to
	/// be used for WebSocket-initiated requests.
	pub session_token: String,
	/// The IP address and port of the client
	pub client_addr: SocketAddr,
}

impl WebSocketClient {
	pub fn streams(
		&self,
	) -> (
		&SplitSink<WebSocketStream<TcpStream>, Message>,
		&SplitStream<WebSocketStream<TcpStream>>,
	) {
		(&self.write, &self.read)
	}
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
			write,
			read,
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
		let user = GreetingsUser {
			username: String::from("Splatterxl"),
			nickname: String::from(""),
			id: unsafe { format!("{}", 0i8) },
		};

		unsafe {
			println!("{}: greet {} ({})", addr, &user.username, &user.id);
		}

		let res = write
			.send(Message::Text(
				to_string(&dispatch::<Greetings>(Some(greetings(user)), 1, None, 1))
					.unwrap_or(String::from("")),
			))
			.await;

		if res.is_err() {
			Err(())
		} else {
			Ok(())
		}
	}

	pub async fn handle(client: Self) {
		let mut read = client.read;
		let mut write = client.write;

		unsafe { println!("{}: started listening for messages", &client.client_addr) };

		while let Some(Ok(msg)) = read.next().await {
			match msg {
				Message::Close(frame) => {
					unsafe {
						println!(
							"{}: connection closed by peer: {:?}",
							client.client_addr, frame
						);
					}
					break;
				}
				Message::Text(m) => {
					let res = WebSocketClient::handle_message(&mut write, m).await;

					if res.is_err() {
						write
							.send(Message::Close(Some(DEFAULT_CLOSE_FRAME)))
							.await
							.unwrap_or(());
						break;
					}
				}
				Message::Binary(m) => {
					let txt = String::from_utf8(m);
					let mut string = String::new();

					// why is this necessary? I don't know.
					let _ = &string;

					if let Err(_) = txt {
						write
							.send(Message::Close(Some(PARSE_ERROR)))
							.await
							.unwrap_or(());
						break;
					} else {
						string = txt.unwrap();
					}

					let res = WebSocketClient::handle_message(&mut write, string).await;

					if res.is_err() {
						write
							.send(Message::Close(Some(DEFAULT_CLOSE_FRAME)))
							.await
							.unwrap_or(());
						break;
					}
				}
				_ => todo!(),
			}
		}
	}

	pub(crate) async fn handle_message(
		write: &mut SplitSink<WebSocketStream<TcpStream>, Message>,
		message: String,
	) -> Result<(), Error> {
		write.send(Message::Text(message)).await
	}
}
