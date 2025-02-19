use serde::{Deserialize, Serialize};

use crate::{data::Database, http::HttpResponse};

type Result<T> = crate::Result<T>;

use super::base_model::BaseModel;
#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct SessionSensorData {
    #[serde(default)]
    id: String,
    datetime: String,
    data_blob: String,
}

impl SessionSensorData {
    pub fn new(id: String, datetime: String, data_blob: String) -> Self {
        SessionSensorData {
            id,
            datetime,
            data_blob,
        }
    }

    pub fn get_id(&self) -> &str {
        &self.id
    }

    pub fn get_datetime(&self) -> &str {
        &self.datetime
    }

    pub fn get_blob(&self) -> &str {
        &self.data_blob
    }

    pub fn try_batch_model(
        database: &dyn Database,
        body: Option<serde_json::Value>,
    ) -> crate::http::HttpResponse {
        let msg = Some(" Requires the values \"datapoints\": array [ { \"id\": string, \"datetime\": string, \"data_blob\": string }, ... ]");
        match body {
            Some(json) => match json.get("datapoints") {
                Some(json_value_array) => match json_value_array.as_array() {
                    Some(json_array) => match json_array
                        .iter()
                        .map(|json_value| SessionSensorData::from_json(json_value.clone()))
                        .collect::<std::result::Result<Vec<_>, _>>()
                    {
                        Ok(data) => match database.batch_session_sensor_data(&data) {
                            Ok(_) => HttpResponse::no_content(),
                            Err(_) => HttpResponse::bad_request(msg.unwrap()),
                        },
                        Err(_) => HttpResponse::invalid_body(msg),
                    },
                    None => HttpResponse::invalid_body(msg),
                },
                None => HttpResponse::invalid_body(msg),
            },
            None => HttpResponse::missing_body(msg),
        }
    }
}

impl BaseModel for SessionSensorData {
    const TYPE_NAME: &'static str = "session sensor data";
    const REQUIRED_VALUES: &'static str =
        " Requires values \"datetime\": string and \"data_blob\": string";

    fn is_valid(&self) -> bool {
        !self.id.is_empty() && !self.datetime.is_empty() && !self.data_blob.is_empty()
    }

    fn public_json(&self) -> String {
        format!(
            "{{\"id\":\"{}\", \"datetime\":\"{}\", \"data_blob\":\"{}\"}}",
            self.id, self.datetime, self.data_blob
        )
    }

    fn insert_interface() -> impl FnOnce(&dyn Database, Self) -> Result<Self>
    where
        Self: Sized,
    {
        |database: &dyn Database, session_sensor_data: Self| -> Result<Self> {
            database.insert_session_sensor_data(&session_sensor_data)
        }
    }
}
