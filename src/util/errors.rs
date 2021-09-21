//! Errors used throughout the API

use std::{error::Error, fmt::Display, ptr::addr_of};

use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum Errors {
	// Miscellanous
	UnknownError { action: String, error: String },

	// Users
	UserAlreadyExists,
}

impl Error for Errors {}
impl Display for Errors {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"Error {}\n\tat: {}",
			match &self {
				Errors::UnknownError { action, error } =>
					format!("could not {}: {}", action, error),
				Errors::UserAlreadyExists => String::from("user already exists"),
			},
			addr_of!(self) as usize
		)
	}
}

pub type Result<T> = std::result::Result<T, Errors>;
