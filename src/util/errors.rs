//! Errors used throughout the API

use std::{error::Error, fmt::Display, ptr::addr_of};

use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(tag = "type")]
pub enum Errors {
	// Miscellanous
	UnknownError { action: String, error: String },

	// Users
	UserAlreadyExists,

	// Sessions
	UserDoesNotExist,
}

impl Error for Errors {}
impl Display for Errors {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"Error {}\n\tat: {}",
			match &self {
				Self::UnknownError { action, error } => format!("could not {}: {}", action, error),
				Self::UserAlreadyExists => String::from("user already exists"),
				Self::UserDoesNotExist => String::from("user does not exist"),
			},
			addr_of!(self) as usize
		)
	}
}

pub type Result<T> = std::result::Result<T, Errors>;
