use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct WebSocketMessage<T> {
	pub t: u8,
	pub e: Option<u8>,
	pub d: Option<T>,
	pub n: usize,
	pub s: Option<String>,
}

pub mod dispatches {
	use serde::{Deserialize, Serialize};

	use self::data_types::PartialUser;

	#[derive(Serialize, Deserialize, Debug)]
	pub struct Hello {
		pub exchange_interval: u16,
	}

	#[doc = "aaaaA"]
	#[derive(Serialize, Deserialize, Debug)]
	pub struct Greetings {
		pub user: PartialUser,
	}

	pub mod data_types {
		use serde::{Deserialize, Serialize};

		#[derive(Serialize, Deserialize, Debug)]
		pub struct PartialUser {
			pub username: String,
			pub nickname: String,
			pub id: u32,
		}
	}
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WebSocketSession {
	pub session_token: String,
}

pub mod gen {
	use super::{
		dispatches::{data_types::PartialUser, Greetings, Hello},
		WebSocketMessage,
	};

    pub fn dispatch<T>(data: Option<T>, t: u8, event: Option<u8>, sequence: usize) -> WebSocketMessage<T> {
        WebSocketMessage {
            t,
            e: event,
            d: data,
            n: sequence,
            s: None
        }
    }

	pub fn hello() -> Hello {
			Hello {
				exchange_interval: 60106,
			}
	}

	pub fn greetings(user: PartialUser) -> Greetings {
		Greetings { user }
	}
}
