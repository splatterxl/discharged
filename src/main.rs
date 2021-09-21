// rust-analyzer moment
#![allow(unused_unsafe)]

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde_json;

extern crate colorful;

use colorful::{Color, Colorful};

mod database;
mod http;
mod types;
mod util;
mod ws;

use std::{
	env,
	error::Error,
	process::Command,
};

use database::start as connect_and_setup;

use tokio::{join, net::TcpListener};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
	println!("{}", format!("🚀 Starting Discharged version {} ({})...", 
                            env!("CARGO_PKG_VERSION").bold().color(Color::Yellow),
                            String::from_utf8(Command::new("git")
                                .args(&["log", "-n", "1", "--format=%h"])
                                .output()
                                .expect("failed to get git commit hash: is git installed?").stdout).unwrap().trim_end().dim()));

	ctrlc::set_handler(|| {
		std::process::exit(130);
	}).expect("couldn't set ctrlc handler");

	connect_and_setup().await?;

	join!(launch_ws(), launch_web());

	Ok(())
}

async fn launch_ws() {
	let addr = env::args()
		.nth(1)
		.unwrap_or_else(|| String::from("127.0.0.1:8083"));

	// Create the event loop and TCP listener we'll accept connections on.
	let try_socket = TcpListener::bind(&addr).await;
	let listener = try_socket.expect("Failed to bind");

	unsafe { println!("[{}] listening on: {}", "ws".color(Color::LightBlue), &addr) };

	while let Ok((stream, _)) = listener.accept().await {
		tokio::spawn(ws::accept_connection(stream));
	}
}

/// Also <s>stolen</s> adapted from Revolt
async fn launch_web() {
	http::mount(rocket::build()).launch().await.unwrap()
}
