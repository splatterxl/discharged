use colorful::Colorful;
use mongodb::bson::doc;
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use rocket::{serde::json::Json, Route};

use crate::{
	database::{get_collection, users},
	success,
	types::sessions::{DeviceType, Session, SessionCreatePayload, SessionDevice},
	util::errors::{Errors, Result},
};

macro_rules! create_log {
    ($($arg:tt)*) => ({
        println!("[{} -> {} /sessions] {}", "http".light_red(), "POST".light_yellow(), format!($($arg)*));
    })
}

#[post("/", data = "<session_data>")]
pub fn create_session(session_data: Json<SessionCreatePayload>) -> Result<String> {
	let session_data = session_data.into_inner();

	create_log!("recieved session create request");

	let user = users::get("0");

	create_log!(
		"resolving user (token: {})...",
		session_data.token.clone().light_gray()
	);

	if let Err(what) = &user {
		Err(Errors::UnknownError {
			action: "resolve_user".to_string(),
			error: format!("{}", what),
		})
	} else {
		let user = user.unwrap();

		if user.is_none() {
			create_log!("invalid user provided");
			Err(Errors::UserDoesNotExist)
		} else {
			let user = user.unwrap();
			create_log!("user found: {} ({})", &user.username, &user.id);

			let mut rng = thread_rng();

			let s: String = (&mut rng)
				.sample_iter(Alphanumeric)
				.take(29)
				.map(char::from)
				.collect();

			drop(rng);

			let session_result = Session {
				uid: (&user.id).parse::<usize>().unwrap(),
				friendly_name: session_data.friendly_name,
				device: SessionDevice {
					_type: DeviceType::PC,
				},
				session_token: s,
			};

			create_log!(
				"creating session with token {}...",
				session_result.session_token.clone().light_gray()
			);

			let session = get_collection::<Session>("sessions").insert_one(&session_result, None);

			match session {
				Err(why) => {
					dbg!(&why);
					Err(Errors::UnknownError {
						action: "create_session".to_string(),
						error: format!("{}", why),
					})
				}
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
