use serde::{Deserialize, Serialize};

use crate::Database;

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
            self.id, self.sensor_type
        )
    }

    fn insert_interface() -> impl FnOnce(&dyn Database, Self) -> Result<Self>
    where
        Self: Sized,
    {
        |database: &dyn Database, sensor: Self| -> Result<Self> { database.insert_sensor(&sensor) }
    }
}
