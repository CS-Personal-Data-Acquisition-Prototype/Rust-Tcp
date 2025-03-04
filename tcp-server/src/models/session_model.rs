use serde::{Deserialize, Serialize};

use crate::{data::Database, http::HttpPath};

type Result<T> = crate::Result<T>;

use super::base_model::BaseModel;
#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct Session {
    #[serde(default)]
    id: String,
    username: String,
}

impl Session {
    pub fn new(id: String, username: String) -> Self {
        Session { id, username }
    }
    pub fn empty() -> Self {
        Self::new(String::new(), String::new())
    }
    pub fn get_id(&self) -> &str {
        &self.id
    }

    pub fn get_username(&self) -> &str {
        &self.username
    }
}

impl BaseModel for Session {
    const TYPE_NAME: &'static str = "session";
    const REQUIRED_VALUES: &'static str = " Requires value \"username\": string";

    fn is_valid(&self) -> bool {
        !self.id.is_empty() && !self.username.is_empty()
    }

    fn public_json(&self) -> String {
        format!(
            "{{\"session_id\":\"{}\", \"username\":\"{}\"}}",
            self.id, self.username
        )
    }

    fn fill_from(&mut self, other: &Self) {
        if self.id.is_empty() {
            self.id = other.get_id().to_string()
        }
        if self.username.is_empty() {
            self.username = other.get_username().to_string()
        }
    }

    fn insert_interface() -> impl FnOnce(&dyn Database, Self) -> Result<Self>
    where
        Self: Sized,
    {
        |database: &dyn Database, session: Self| -> Result<Self> {
            database.insert_session(&session)
        }
    }

    fn update_interface() -> impl FnOnce(&dyn Database, &str, Self) -> Result<Self>
    where
        Self: Sized,
    {
        |database: &dyn Database, subpath: &str, updated_session: Self| -> Result<Self> {
            match HttpPath::subsection(&subpath, 0) {
                Some(id) => database.update_session(id, &updated_session),
                None => Err(format!("Missing identifier in path: {subpath}")),
            }
        }
    }

    fn delete_interface() -> impl FnOnce(&dyn Database, &str) -> Result<()>
    where
        Self: Sized,
    {
        |database: &dyn Database, subpath: &str| -> Result<()> {
            match HttpPath::subsection(&subpath, 0) {
                Some(id) => database.delete_session(id),
                None => Err(format!("Missing identifier in path: {subpath}")),
            }
        }
    }
}
