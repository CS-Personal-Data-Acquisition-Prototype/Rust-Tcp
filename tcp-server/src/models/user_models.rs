use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct User {
    username: String,
    password_hash: String,
}

impl User {
    pub fn new(username: String, password_hash: String) -> Self {
        User {
            username,
            password_hash,
        }
    }

    pub fn is_valid(&self) -> bool {
        !self.username.is_empty() && !self.password_hash.is_empty()
    }

    pub fn public_json(&self) -> String {
        format!("{{\"username\":\"{}\"}}", self.username)
    }

    pub fn get_username(&self) -> String {
        self.username.clone()
    }
}
