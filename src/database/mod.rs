//! Shamelessly stolen from Revolt.chat's [`delta`] (backend) repository.
//!
//! [`delta`]: https://github.com/revoltchat/delta
use std::error::Error;

use mongodb::{
	bson::doc,
	options::{ClientOptions, ResolverConfig},
	sync::{Client, Collection, Database},
};
use tokio::sync::OnceCell;

use crate::{types::database::User, util::variables::MONGO_URI};

pub mod users;

lazy_static! {
	static ref DBCONN: OnceCell<Client> = OnceCell::new();
}

pub async fn start() -> Result<(), Box<dyn Error>> {
	connect().await?;
	setup()?;

	Ok(())
}

pub async fn connect() -> mongodb::error::Result<()> {
	let client = Client::with_options(ClientOptions::parse_with_resolver_config(
		&*MONGO_URI,
		ResolverConfig::cloudflare(),
	)?)?;

	client
		.database("admin")
		.run_command(doc! {"ping": 1}, None)?;

	unsafe {
		println!("Successfully connected to MongoDB cluster");
	}

	DBCONN.set(client).expect("Couldn't set Client to DBCONN");

	Ok(())
}

pub fn setup() -> Result<(), Box<dyn Error>> {
	// users
	let admin = users::get("0")?;

	match admin {
		Some(_) => {
			println!("Administrator user is already set, skipping setup");
		}
		None => {
			println!("Administrator user not found, inserting...");

			users::create(User {
				id: String::from("0"),
				username: String::from("Discharged"),
				nickname: None,
			})?;

			println!("-> Success");
		}
	}

	println!("User setup complete");

	Ok(())
}

pub fn get_connection() -> &'static Client {
	DBCONN.get().unwrap()
}

pub fn get_database() -> Database {
	get_connection().database("discharged")
}

pub fn get_collection<T>(collection: &str) -> Collection<T> {
	get_database().collection::<T>(collection)
}
