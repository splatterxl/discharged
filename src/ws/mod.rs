use std::fmt::Debug;

use colorful::Colorful;
use futures_util::{SinkExt, StreamExt};
use mongodb::bson::doc;
use serde::Deserialize;
use serde_json::{from_str, to_string};
use tokio::net::TcpStream;
use tokio_tungstenite::tungstenite::Message;

use crate::{
	database::get_collection,
	types::sessions::Session,
	ws::{
		client::WebSocketClient,
		errors::{DEFAULT_CLOSE_FRAME, INVALID_TOKEN, PARSE_ERROR},
		schemas::{
			dispatches::Hello,
			gen::{dispatch, hello},
			WebSocketMessage, WebSocketSession,
		},
	},
};

use self::schemas::constants::Opcodes;

pub mod client;
pub mod errors;
pub mod schemas;

pub fn validate<'a, T>(message: &'a str) -> Result<(), ()>
where
	T: Deserialize<'a> + Debug,
{
	let obj = from_str::<'a, WebSocketMessage<T>>(message);
	let opcode = if obj.is_ok() {
		Opcodes::get(obj.as_ref().unwrap().t)
	} else {
		Err(String::new())
	};

	if obj.is_err() || opcode.is_err() {
		println!("{:#?} {:#?}", obj, opcode);
		Err(())
	} else {
		Ok(())
	}
}
pub async fn accept_connection(stream: TcpStream) {
	let addr = stream.peer_addr();

	if addr.is_err() {
		println!("[{}] connection request has no peer address", "ws".blue());
		return;
	}

	let addr = addr.unwrap();

	println!(
		"[{}] new connect request (peer address: {})",
		"ws".blue(),
		addr
	);

	let ws_stream = tokio_tungstenite::accept_async(stream)
		.await
		.expect("Error during the websocket handshake occurred");

	println!(
		"[{}] {}: WebSocket connection succeeded",
		"ws".blue(),
		&addr
	);

	let (mut write, mut read) = ws_stream.split();

	let hello = to_string(&dispatch::<Hello>(Some(hello()), 0, None, 0))
		.unwrap_or_else(|_| String::from("null"));

	if hello == "null" {
		println!("[{}] {}: couldn't stringify hello", "ws".blue(), addr);
		write
			.send(Message::Close(Some(DEFAULT_CLOSE_FRAME)))
			.await
			.expect("couldn't close");
	} else {
		write
			.send(Message::Text(hello))
			.await
			.expect("couldn't send HELLO");

		if let Some(Ok(mut msg)) = read.next().await {
			if let Message::Binary(bin) = msg {
				msg = Message::Text(String::from_utf8(bin).unwrap_or_else(|_| String::from("")));
			}

			match msg {
				Message::Text(mut msg) => {
					msg = String::from(msg.trim());

					if validate::<WebSocketSession>(&msg).is_err() {
						println!("[{}] {}: invalid opening packet", "ws".blue(), addr);
						write
							.send(Message::Close(Some(PARSE_ERROR)))
							.await
							.expect("couldn't close socket");
						return;
					}

					let msg =
						from_str::<WebSocketMessage<WebSocketSession>>(&msg.as_str()).unwrap();

					let mut session_token = String::from("");

					let _ = &session_token;

					if let Some(data) = msg.d {
						session_token = data.session_token;
					} else {
						println!("[{}] {}: invalid opening packet", "ws".blue(), addr);
						write
							.send(Message::Close(Some(PARSE_ERROR)))
							.await
							.expect("couldn't close socket");
						return;
					}

					println!(
						"[{}] {}: session token \"{}\" received, validating...",
						"ws".blue(),
						&addr,
						&session_token.clone().light_gray()
					);

					match get_collection::<Session>("sessions")
						.find_one(doc! { "session_token": session_token.clone() }, None)
					{
						Ok(res) => match res {
							Some(session) => {
								println!("[{}] {}: session token validated", "ws".blue(), &addr);

								WebSocketClient::new((write, read), session, addr).await;
							}
							None => {
								write
									.send(Message::Close(Some(INVALID_TOKEN)))
									.await
									.unwrap_or(());
								println!("[{}] {}: invalid token", "ws".blue(), &addr)
							}
						},
						Err(why) => {
							write
								.send(Message::Close(Some(DEFAULT_CLOSE_FRAME)))
								.await
								.unwrap_or(());
							println!("[{}] unknown error occurred while resolving session token {}: {:#?}", "ws".blue(), session_token.light_gray(), why)
						}
					};
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
