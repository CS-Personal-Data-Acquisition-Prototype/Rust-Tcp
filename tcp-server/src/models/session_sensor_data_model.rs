use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{
    data::Database,
    http::{HttpPath, HttpResponse},
};

type Result<T> = crate::Result<T>;

use super::base_model::BaseModel;
#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct SessionSensorData {
    #[serde(default)]
    id: i64,
    datetime: String,
    data_blob: String,
}

impl SessionSensorData {
    pub fn new(id: i64, datetime: String, data_blob: String) -> Self {
        SessionSensorData {
            id,
            datetime,
            data_blob,
        }
    }

    #[allow(unused)]
    pub fn empty() -> Self {
        Self::new(-1, String::new(), String::new())
    }

    pub fn get_id(&self) -> &i64 {
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
                            Ok(_) => {
                                println!("Data batch recieved to DB\n");
                                HttpResponse::no_content()
                            }
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
        self.id >= 0 && !self.datetime.is_empty() && !self.data_blob.is_empty()
    }

    fn public_json(&self) -> String {
        json!({
            "id": self.id,
            "datetime": self.datetime,
            "data_blob": self.data_blob
        })
        .to_string()
    }

    fn fill_from(&mut self, other: &Self) {
        if self.id == -1 {
            self.id = other.get_id().clone()
        }
        if self.datetime.is_empty() {
            self.datetime = other.get_datetime().to_string()
        }
        if self.data_blob.is_empty() {
            self.data_blob = other.get_blob().to_string()
        }
    }

    fn insert_interface() -> impl FnOnce(&dyn Database, Self) -> Result<Self>
    where
        Self: Sized,
    {
        |database: &dyn Database, session_sensor_data: Self| -> Result<Self> {
            database.insert_session_sensor_data(&session_sensor_data)
        }
    }

    fn update_interface() -> impl FnOnce(&dyn Database, &str, Self) -> Result<Self>
    where
        Self: Sized,
    {
        |database: &dyn Database,
         subpath: &str,
         updated_session_sensor_datapoint: Self|
         -> Result<Self> {
            match HttpPath::subsection(&subpath, 0) {
                Some(id) => match HttpPath::subsection(&subpath, 1) {
                    Some(datetime) => match id.parse::<i64>() {
                        Ok(id) => database.update_session_sensor_datapoint(
                            id,
                            datetime,
                            &updated_session_sensor_datapoint,
                        ),
                        Err(e) => Err(format!("Failed to parse id to i64: {e}")),
                    },
                    None => Err(format!("Missing identifier in path: {subpath}")),
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
                Some(id) => match HttpPath::subsection(&subpath, 1) {
                    Some(datetime) => match id.parse::<i64>() {
                    Ok(id) => database.delete_session_sensor_datapoint(id, datetime),
                    Err(e) => Err(format!("Failed to parse id to i64: {e}")),
                },
                    None => Err(format!("Missing identifier in path: {subpath}")),
                },
                None => Err(format!("Missing identifier in path: {subpath}")),
            }
        }
    }
}
