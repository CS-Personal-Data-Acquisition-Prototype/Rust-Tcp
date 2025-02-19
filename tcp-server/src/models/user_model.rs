use serde::{Deserialize, Serialize};

use crate::data::Database;

type Result<T> = crate::Result<T>;

use super::base_model::BaseModel;

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
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

    pub fn get_username(&self) -> String {
        self.username.clone()
    }
}

impl BaseModel for User {
    const TYPE_NAME: &'static str = "user";
    const REQUIRED_VALUES: &'static str =
        " Requires values \"username\": string and \"password_hash\": string";

    fn is_valid(&self) -> bool {
        !self.username.is_empty() && !self.password_hash.is_empty()
    }

    fn public_json(&self) -> String {
        format!("{{\"username\":\"{}\"}}", self.username)
    }

    fn insert_interface() -> impl FnOnce(&dyn Database, Self) -> Result<Self>
    where
        Self: Sized,
    {
        |database: &dyn Database, user: Self| -> Result<Self> { database.insert_user(&user) }
    }
}
