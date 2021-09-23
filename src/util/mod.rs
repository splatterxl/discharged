use std::{
	error::Error,
	fs,
	process::{self, Command},
};

use tokio::time::Duration;

pub mod variables {
	lazy_static! {
		pub static ref MONGO_URI: String = String::from(env!("MONGO"));
	}
}

pub mod errors;

/// Taken from process_uptime crate
pub fn uptime() -> Result<Duration, Box<dyn Error>> {
	let pid = process::id();

	Ok(match get_ps_etime(pid) {
		Ok(etime) => Duration::from_secs(etime),
		Err(_) => {
			// Fallback to getting from /proc/{}
			let metadata = fs::metadata(format!("/proc/{}", pid))?;
			metadata.modified()?.elapsed()?
		}
	})
}

fn get_ps_etime(pid: u32) -> Result<u64, Box<dyn Error>> {
	let output = Command::new("sh")
		.arg("-c")
		.arg(format!("ps -o etimes -p {} --no-headers", pid).as_str())
		.output()?;

	let mut uptime_string = std::str::from_utf8(output.stdout.as_slice())?.to_string();

	if uptime_string.ends_with('\n') {
		uptime_string.pop();
		if uptime_string.ends_with('\r') {
			uptime_string.pop();
		}
	}

	match uptime_string.parse::<u64>() {
		Ok(uptime) => Ok(uptime),
		Err(err) => Err(Box::new(err)),
	}
}
