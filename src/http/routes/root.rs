use rocket::Route;

use crate::util::{
	errors::{Errors, Result},
	uptime,
};

#[get("/")]
pub fn ping() -> Result<String> {
	match uptime() {
		Err(err) => Err(Errors::UnknownError {
			action: "query_uptime".to_string(),
			error: format!("{}", err),
		}),
		Ok(uptime) => Ok(json!({ "uptime": uptime }).to_string()),
	}
}

pub fn all() -> Vec<Route> {
	routes![ping]
}
