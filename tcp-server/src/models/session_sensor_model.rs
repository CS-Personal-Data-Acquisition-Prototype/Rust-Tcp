/*
Copyright 2025 CS 462 Personal Data Acquisition Prototype Group
    
Licensed under the Apache License, Version 2.0 (the "License"); you may not use this file except in compliance with the License.
You may obtain a copy of the License at http://www.apache.org/licenses/LICENSE-2.0
Unless required by applicable law or agreed to in writing, software distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied. See the License for the specific language governing permissions and limitations under the License.
*/
use serde::{Deserialize, Serialize};

type Result<T> = crate::Result<T>;

use crate::{data::Database, http::HttpPath};

use super::base_model::BaseModel;
#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct SessionSensor {
    #[serde(default)]
    id: i64,
    session_id: i64,
    sensor_id: i64,
}

impl SessionSensor {
    pub fn new(id: i64, session_id: i64, sensor_id: i64) -> Self {
        SessionSensor {
            id,
            session_id,
            sensor_id,
        }
    }

    #[allow(unused)]
    pub fn empty() -> Self {
        Self::new(-1, -1, -1)
    }

    pub fn get_id(&self) -> &i64 {
        &self.id
    }

    pub fn get_session_id(&self) -> &i64 {
        &self.session_id
    }

    pub fn get_sensor_id(&self) -> &i64 {
        &self.sensor_id
    }
}

impl BaseModel for SessionSensor {
    const TYPE_NAME: &'static str = "session sensor";
    const REQUIRED_VALUES: &'static str =
        " Requires values \"session_id\": string and \"sensor_id\": string";

    fn is_valid(&self) -> bool {
        self.id >= 0 && self.session_id >= 0 && self.sensor_id >= 0
    }

    fn public_json(&self) -> String {
        format!(
            "{{\"id\":\"{}\", \"session_id\":\"{}\", \"sensor_id\":\"{}\"}}",
            self.id, self.session_id, self.sensor_id
        )
    }

    fn fill_from(&mut self, other: &Self) {
        if self.id == -1 {
            self.id = other.get_id().clone()
        }
        if self.session_id == -1 {
            self.session_id = other.session_id.clone()
        }
        if self.sensor_id == -1 {
            self.sensor_id = other.sensor_id.clone()
        }
    }

    fn insert_interface() -> impl FnOnce(&dyn Database, Self) -> Result<Self>
    where
        Self: Sized,
    {
        |database: &dyn Database, session_sensor: Self| -> Result<Self> {
            database.insert_session_sensor(&session_sensor)
        }
    }

    fn update_interface() -> impl FnOnce(&dyn Database, &str, Self) -> Result<Self>
    where
        Self: Sized,
    {
        |database: &dyn Database, subpath: &str, updated_session_sensor: Self| -> Result<Self> {
            match HttpPath::subsection(&subpath, 0) {
                Some(id) => match id.parse::<i64>() {
                    Ok(id) => database.update_session_sensor(id, &updated_session_sensor),
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
                    Ok(id) => database.delete_session_sensor(id),
                    Err(e) => Err(format!("Failed to parse id to i64: {e}")),
                },
                None => Err(format!("Missing identifier in path: {subpath}")),
            }
        }
    }
}
