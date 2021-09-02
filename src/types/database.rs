use serde::{Deserialize, Serialize}

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    id: usize;
    username: String,
    nickname: String,
}
