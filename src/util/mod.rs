pub mod variables {
	pub const MONGO_URI: String =
		String::from(option_env!("MONGO").expect("MongoDB url not specified"));
}
