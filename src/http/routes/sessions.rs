use colorful::Colorful;
use mongodb::bson::doc;
use rocket::{serde::json::Json, Route};

use crate::{
	database::{get_collection, users},
	success,
	types::sessions::{DeviceType, Session, SessionCreatePayload, SessionDevice},
	util::errors::{Errors, Result},
};

#[post("/", data = "<session_data>")]
pub fn create_session(session_data: Json<SessionCreatePayload>) -> Result<String> {
	let session_data = session_data.into_inner();

	println!("recieved session create request");

	let user = users::get("0");

	println!("resolving user...");

	if let Err(what) = &user {
		Err(Errors::UnknownError {
			action: "resolve_user".to_string(),
			error: format!("{}", what),
		})
	} else {
		let user = user.unwrap();

		if user.is_none() {
			println!("invalid user provided");
			Err(Errors::UserDoesNotExist)
		} else {
			let user = user.unwrap();
			println!(
				"User found...\n\tID: {}\n\tUsername: {}",
				&user.id, &user.username
			);

			let session_result = Session {
				user,
				friendly_name: session_data.friendly_name,
				device: SessionDevice {
					_type: DeviceType::PC,
				},
				session_token: "dfavhnwegfi9ewfnwref".to_string(),
			};

			println!("creating session...");

			let session = get_collection::<Session>("sessions").insert_one(&session_result, None);

			match session {
				Err(why) => Err(Errors::UnknownError {
					action: "create_session".to_string(),
					error: format!("{}", why),
				}),
				Ok(_) => {
					success!();
					Ok(json!(session_result).to_string())
				}
			}
		}
	}
}

pub fn all() -> Vec<Route> {
	routes![create_session]
}
