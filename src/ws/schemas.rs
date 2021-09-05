use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct WebSocketMessage<T> {
	pub t: u8,
	pub e: Option<u8>,
	pub d: Option<T>,
	pub n: usize,
}

pub mod dispatches {
	use serde::{Deserialize, Serialize};

	use crate::types::ws::GreetingsUser;

	#[derive(Serialize, Deserialize, Debug)]
	pub struct Hello {
		pub exchange_interval: u16,
	}

	#[doc = "aaaaA"]
	#[derive(Serialize, Deserialize, Debug)]
	pub struct Greetings {
		pub user: GreetingsUser,
	}
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WebSocketSession {
	pub session_token: String,
}

pub mod gen {
	use super::{
		dispatches::{Greetings, Hello},
		WebSocketMessage,
	};

	use crate::types::ws::GreetingsUser;

	pub fn dispatch<T>(
		data: Option<T>,
		t: u8,
		event: Option<u8>,
		sequence: usize,
	) -> WebSocketMessage<T> {
		WebSocketMessage {
			t,
			e: event,
			d: data,
			n: sequence,
		}
	}

	pub fn hello() -> Hello {
		Hello {
			exchange_interval: 60106,
		}
	}

	pub fn greetings(user: GreetingsUser) -> Greetings {
		Greetings { user }
	}
}

pub mod constants {
	#[derive(Debug)]
	pub enum Opcodes {
		Hello,
		Greetings,
		Authenticate,
	}

	#[allow(dead_code)]
	impl Opcodes {
		pub fn hello() -> u8 {
			0
		}

		pub fn greetings() -> u8 {
			1
		}

		pub fn authenticate() -> u8 {
			2
		}

		////////////////////////////

		pub fn get(num: u8) -> Result<Self, String> {
			return match num {
				0 => Ok(Self::Hello),
				1 => Ok(Self::Greetings),
				2 => Ok(Self::Authenticate),
				_ => unsafe {
					Err(format!(
						"Opcodes::get() called with an invalid number: {}",
						num
					))
				},
			};
		}
	}
}
