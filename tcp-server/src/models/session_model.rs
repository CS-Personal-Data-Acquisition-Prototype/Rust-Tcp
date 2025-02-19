use serde::{Deserialize, Serialize};

use crate::data::Database;

type Result<T> = crate::Result<T>;

use super::base_model::BaseModel;
#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct Session {
    #[serde(default)]
    session_id: String,
    username: String,
}

impl Session {
    pub fn new(session_id: String, username: String) -> Self {
        Session {
            session_id,
            username,
        }
    }
    pub fn get_id(&self) -> &str {
        &self.session_id
    }

    pub fn get_username(&self) -> &str {
        &self.username
    }
}

impl BaseModel for Session {
    const TYPE_NAME: &'static str = "session";
    const REQUIRED_VALUES: &'static str = " Requires value \"username\": string";

    fn is_valid(&self) -> bool {
        !self.session_id.is_empty() && !self.username.is_empty()
    }

    fn public_json(&self) -> String {
        format!(
            "{{\"session_id\":\"{}\", \"username\":\"{}\"}}",
            self.session_id, self.username
        )
    }

    fn insert_interface() -> impl FnOnce(&dyn Database, Self) -> Result<Self>
    where
        Self: Sized,
    {
        |database: &dyn Database, session: Self| -> Result<Self> {
            database.insert_session(&session)
        }
    }
}
