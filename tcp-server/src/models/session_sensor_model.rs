use serde::{Deserialize, Serialize};

type Result<T> = crate::Result<T>;

use crate::{data::Database, http::HttpPath};

use super::base_model::BaseModel;
#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct SessionSensor {
    #[serde(default)]
    id: String,
    session_id: String,
    sensor_id: String,
}

impl SessionSensor {
    pub fn new(id: String, session_id: String, sensor_id: String) -> Self {
        SessionSensor {
            id,
            session_id,
            sensor_id,
        }
    }

    pub fn empty() -> Self {
        Self::new(String::new(), String::new(), String::new())
    }

    pub fn get_id(&self) -> &str {
        &self.id
    }

    pub fn get_session_id(&self) -> &str {
        &self.session_id
    }

    pub fn get_sensor_id(&self) -> &str {
        &self.sensor_id
    }
}

impl BaseModel for SessionSensor {
    const TYPE_NAME: &'static str = "session sensor";
    const REQUIRED_VALUES: &'static str =
        " Requires values \"session_id\": string and \"sensor_id\": string";

    fn is_valid(&self) -> bool {
        !self.id.is_empty() && !self.session_id.is_empty() && !self.sensor_id.is_empty()
    }

    fn public_json(&self) -> String {
        format!(
            "{{\"id\":\"{}\", \"session_id\":\"{}\", \"sensor_id\":\"{}\"}}",
            self.id, self.session_id, self.sensor_id
        )
    }

    fn fill_from(&mut self, other: &Self) {
        if self.id.is_empty() {
            self.id = other.get_id().to_string()
        }
        if self.session_id.is_empty() {
            self.session_id = other.session_id.to_string()
        }
        if self.sensor_id.is_empty() {
            self.sensor_id = other.sensor_id.to_string()
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
                Some(id) => database.update_session_sensor(id, &updated_session_sensor),
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
                Some(id) => database.delete_session_sensor(id),
                None => Err(format!("Missing identifier in path: {subpath}")),
            }
        }
    }
}
