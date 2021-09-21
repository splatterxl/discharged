use std::io::Cursor;

use rocket::{
	http::{ContentType, Status},
	response::{Responder, Result},
	Build, Request, Response, Rocket,
};

use crate::util::errors::Errors;

mod routes;

pub fn mount(rocket: Rocket<Build>) -> Rocket<Build> {
	rocket.mount("/", routes::root::all())
}

impl<'r, 'o: 'r> Responder<'r, 'o> for Errors {
	fn respond_to(self, request: &'r Request<'_>) -> Result<'o> {
		let string = json!(&self).to_string();

		let status = match &self {
			&Self::UnknownError { .. } => Status::InternalServerError,
			&Self::UserAlreadyExists => Status::Conflict,
		};

		Response::build()
			.sized_body(string.len(), Cursor::new(string))
			.header(ContentType::JSON)
			.status(status)
			.ok()
	}
}
