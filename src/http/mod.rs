use std::io::Cursor;

use rocket::{
	http::{ContentType, Status},
	response::{Responder, Result},
	Build, Request, Response, Rocket,
};

use crate::util::errors::Errors;

mod routes;

pub fn mount(rocket: Rocket<Build>) -> Rocket<Build> {
	rocket
		.mount("/", routes::root::all())
		.mount("/sessions", routes::sessions::all())
}

impl<'r, 'o: 'r> Responder<'r, 'o> for Errors {
	fn respond_to(self, _request: &'r Request<'_>) -> Result<'o> {
		let status = match &self {
			&Self::UnknownError { .. } => Status::InternalServerError,
			&Self::UserAlreadyExists => Status::Conflict,
			&Self::UserDoesNotExist => Status::BadRequest,
			&Self::BadRequest => Status::BadRequest,
		};

		let string = json!(self).to_string();

		Response::build()
			.sized_body(string.len(), Cursor::new(string))
			.header(ContentType::JSON)
			.status(status)
			.ok()
	}
}
