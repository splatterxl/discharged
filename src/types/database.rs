use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
	#[serde(rename = "_id")]
	id: String,
	username: String,
	nickname: String,
}
