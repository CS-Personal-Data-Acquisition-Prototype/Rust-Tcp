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

    #[allow(unused)]
    pub fn empty() -> Self {
        Self::new(String::new(), String::new())
    }

    pub fn get_username(&self) -> &str {
        &self.username
    }

    pub fn get_password_hash(&self) -> &str {
        &self.password_hash
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

    fn fill_from(&mut self, other: &Self) {
        if self.username.is_empty() {
            self.username = other.get_username().to_string()
        }
        if self.password_hash.is_empty() {
            self.password_hash = other.get_password_hash().to_string()
        }
    }

    fn insert_interface() -> impl FnOnce(&dyn Database, Self) -> Result<Self>
    where
        Self: Sized,
    {
        |database: &dyn Database, user: Self| -> Result<Self> { database.insert_user(&user) }
    }

    fn update_interface() -> impl FnOnce(&dyn Database, &str, Self) -> Result<Self>
    where
        Self: Sized,
    {
        |database: &dyn Database, subpath: &str, updated_user: Self| -> Result<Self> {
            match HttpPath::subsection(&subpath, 0) {
                Some(username) => database.update_user(username, &updated_user),
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
                Some(username) => database.delete_user(username),
                None => Err(format!("Missing identifier in path: {subpath}")),
            }
        }
    }
}
