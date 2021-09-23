pub mod database;
pub mod ws;
pub mod sessions {
	use rocket::{
		data::{FromData, Limits},
		http::Status,
		outcome::Outcome,
		Data, Request,
	};
	use serde::{Deserialize, Serialize};

	use crate::util::errors::Errors;

	use super::database::User;

	#[derive(Serialize, Deserialize)]
	pub struct SessionCreatePayload {
		pub token: Option<String>,
		pub friendly_name: String,
	}

	#[derive(Serialize, Deserialize)]
	pub struct Session {
		pub user: User,
		pub friendly_name: String,
		pub device: SessionDevice,
		#[serde(rename = "_id")]
		pub session_token: String,
	}

	#[derive(Serialize, Deserialize)]
	pub struct SessionDevice {
		#[serde(rename = "type")]
		pub _type: DeviceType,
	}

	#[derive(Serialize, Deserialize)]
	#[repr(i32)]
	pub enum DeviceType {
		PC = 1,
		Android = 2,
		Ios = 3,
		Browser = 4,
	}
}
