use mongodb::bson::doc;

use crate::{types::database::User, util::errors::Errors};

use super::get_collection;

pub fn get(id: &str) -> mongodb::error::Result<Option<User>> {
	get_collection::<User>("users").find_one(
		doc! {
		  "_id": String::from(id)
		},
		None,
	)
}

pub fn create(data: User) -> Result<(), Errors> {
	match get(&data.id) {
		Ok(user) => {
			if user.is_some() {
				Err(Errors::UserAlreadyExists)
			} else {
				if let Err(err) = get_collection::<User>("users").insert_one(&data, None) {
					return Err(Errors::UnknownError {
						action: String::from("create user"),
						error: format!("{}", err),
					});
				}
				println!("Created user {} (id {})", data.username, data.id);
				Ok(())
			}
		}
		Err(err) => Err(Errors::UnknownError {
			action: String::from("create"),
			error: format!("{}", err),
		}),
	}
}
