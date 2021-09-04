pub mod variables {
    lazy_static! {
        pub static ref MONGO_URI: String =
            String::from(option_env!("MONGO").expect("MongoDB url not specified"));
    }
}
