use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
	#[serde(rename = "_id")]
	pub id: String,
	pub username: String,
	pub nickname: Option<String>,
}
