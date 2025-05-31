/*
Copyright 2025 CS 462 Personal Data Acquisition Prototype Group
    
Licensed under the Apache License, Version 2.0 (the "License"); you may not use this file except in compliance with the License.
You may obtain a copy of the License at http://www.apache.org/licenses/LICENSE-2.0
Unless required by applicable law or agreed to in writing, software distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied. See the License for the specific language governing permissions and limitations under the License.
*/
use serde::{Deserialize, Serialize};

use crate::{data::Database, http::HttpPath};

type Result<T> = crate::Result<T>;

use super::base_model::BaseModel;
#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct Session {
    #[serde(default)]
    id: i64,
    username: String,
}

impl Session {
    pub fn new(id: i64, username: String) -> Self {
        Session { id, username }
    }
    #[allow(unused)]
    pub fn empty() -> Self {
        Self::new(-1, String::new())
    }
    pub fn get_id(&self) -> &i64 {
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
        self.id >= 0 && !self.username.is_empty()
    }

    fn public_json(&self) -> String {
        format!(
            "{{\"session_id\":\"{}\", \"username\":\"{}\"}}",
            self.id, self.username
        )
    }

    fn fill_from(&mut self, other: &Self) {
        if self.id == -1 {
            self.id = other.get_id().clone()
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
                Some(id) => match id.parse::<i64>() {
                    Ok(id) => database.update_session(id, &updated_session),
                    Err(e) => Err(format!("Failed to parse id to i64: {e}")),
                },
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
                Some(id) => match id.parse::<i64>() {
                    Ok(id) => database.delete_session(id),
                    Err(e) => Err(format!("Failed to parse id to i64: {e}")),
                },
                None => Err(format!("Missing identifier in path: {subpath}")),
            }
        }
    }
}
