use mongodb::bson::doc;

use crate::types::database::User;

use super::get_collection;

pub fn get(id: String) -> mongodb::error::Result<Option<User>> {
	get_collection::<User>("users").find_one(
		doc! {
		  "_id": id
		},
		None,
	)
}
