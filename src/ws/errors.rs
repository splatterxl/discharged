use std::borrow::Cow;

use tokio_tungstenite::tungstenite::protocol::{frame::coding::CloseCode, CloseFrame};

pub const DEFAULT_CLOSE_FRAME: CloseFrame = CloseFrame {
	code: CloseCode::Library(1001),
	reason: Cow::Borrowed("Unknown Error"),
};

pub const PARSE_ERROR: CloseFrame = CloseFrame {
	code: CloseCode::Library(1002),
	reason: Cow::Borrowed("Parse error, please @resume when ready"),
};

pub const INVALID_TOKEN: CloseFrame = CloseFrame {
	code: CloseCode::Library(1003),
	reason: Cow::Borrowed("Invalid token"),
};
