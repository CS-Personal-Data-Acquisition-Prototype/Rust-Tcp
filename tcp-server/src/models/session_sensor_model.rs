use serde::{Deserialize, Serialize};

type Result<T> = crate::Result<T>;

use crate::data::Database;

use super::base_model::BaseModel;
#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct SessionSensor {
    #[serde(default)]
    session_sensor_id: String,
    session_id: String,
    sensor_id: String,
}

impl SessionSensor {
    pub fn new(session_sensor_id: String, session_id: String, sensor_id: String) -> Self {
        SessionSensor {
            session_sensor_id,
            session_id,
            sensor_id,
        }
    }

    pub fn get_id(&self) -> &str {
        &self.session_sensor_id
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
        !self.session_sensor_id.is_empty()
            && !self.session_id.is_empty()
            && !self.sensor_id.is_empty()
    }

    fn public_json(&self) -> String {
        format!(
            "{{\"id\":\"{}\", \"session_id\":\"{}\", \"sensor_id\":\"{}\"}}",
            self.session_sensor_id, self.session_id, self.sensor_id
        )
    }

    fn insert_interface() -> impl FnOnce(&dyn Database, Self) -> Result<Self>
    where
        Self: Sized,
    {
        |database: &dyn Database, session_sensor: Self| -> Result<Self> {
            database.insert_session_sensor(&session_sensor)
        }
    }
}
