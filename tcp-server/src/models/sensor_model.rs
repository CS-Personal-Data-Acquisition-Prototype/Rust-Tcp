use serde::{Deserialize, Serialize};

use crate::{http::HttpPath, Database};

type Result<T> = crate::Result<T>;

use super::base_model::BaseModel;
#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct Sensor {
    #[serde(default)]
    id: String,
    #[serde(rename = "type")]
    sensor_type: String,
}

impl Sensor {
    pub fn new(id: String, sensor_type: String) -> Self {
        Sensor { id, sensor_type }
    }
    pub fn empty() -> Self {
        Self::new(String::new(), String::new())
    }
    pub fn get_id(&self) -> &str {
        &self.id
    }
    pub fn get_sensor_type(&self) -> &str {
        &self.sensor_type
    }
}

impl BaseModel for Sensor {
    const TYPE_NAME: &'static str = "sensor";
    const REQUIRED_VALUES: &'static str = " Requires value \"type\": string";

    fn is_valid(&self) -> bool {
        !self.id.is_empty() && !self.sensor_type.is_empty()
    }

    fn public_json(&self) -> String {
        format!(
            "{{\"id\":\"{}\", \"type\":\"{}\"}}",
            self.get_id(),
            self.get_sensor_type()
        )
    }

    fn fill_from(&mut self, other: &Self) {
        if self.id.is_empty() {
            self.id = other.get_id().to_string()
        }
        if self.sensor_type.is_empty() {
            self.sensor_type = other.get_sensor_type().to_string()
        }
    }

    fn insert_interface() -> impl FnOnce(&dyn Database, Self) -> Result<Self>
    where
        Self: Sized,
    {
        |database: &dyn Database, sensor: Self| -> Result<Self> { database.insert_sensor(&sensor) }
    }

    fn update_interface() -> impl FnOnce(&dyn Database, &str, Self) -> Result<Self>
    where
        Self: Sized,
    {
        |database: &dyn Database, subpath: &str, updated_sensor: Self| -> Result<Self> {
            match HttpPath::subsection(&subpath, 0) {
                Some(id) => database.update_sensor(id, &updated_sensor),
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
                Some(id) => database.delete_sensor(id),
                None => Err(format!("Missing identifier in path: {subpath}")),
            }
        }
    }
}
