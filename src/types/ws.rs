use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct GreetingsUser {
	pub username: String,
	pub nickname: String,
	pub id: String,
}
