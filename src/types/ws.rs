use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct HelloUser {
	pub username: String,
	pub nickname: String,
	pub id: u32,
}
