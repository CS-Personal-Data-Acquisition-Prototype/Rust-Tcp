/*
Copyright 2025 CS 462 Personal Data Acquisition Prototype Group
    
Licensed under the Apache License, Version 2.0 (the "License"); you may not use this file except in compliance with the License.
You may obtain a copy of the License at http://www.apache.org/licenses/LICENSE-2.0
Unless required by applicable law or agreed to in writing, software distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied. See the License for the specific language governing permissions and limitations under the License.
*/
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{
    data::Database,
    http::{HttpPath, HttpResponse},
};

type Result<T> = crate::Result<T>;

use super::base_model::BaseModel;
#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct SessionSensorData {
    #[serde(default)]
    id: Option<i64>,
    datetime: String,
    data_blob: Value,
}

impl SessionSensorData {
    pub fn new(id: Option<i64>, datetime: String, data_blob: Value) -> Self {
        SessionSensorData {
            id,
            datetime,
            data_blob,
        }
    }

    #[allow(unused)]
    pub fn empty() -> Self {
        Self::new(None, String::new(), Value::Null)
    }

    pub fn get_id(&self) -> &Option<i64> {
        &self.id
    }

    pub fn get_datetime(&self) -> &str {
        &self.datetime
    }

    pub fn get_blob(&self) -> &Value {
        &self.data_blob
    }

    pub fn try_batch_model(
        database: &dyn Database,
        body: Option<serde_json::Value>,
    ) -> crate::http::HttpResponse {
        let msg = Some(" Requires the values \"datapoints\": array [ { \"id\": i64, \"datetime\": string, \"data_blob\": string }, ... ]");
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
        self.id.is_some() && !self.datetime.is_empty() && self.data_blob.is_object()
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
        if self.id.is_none() {
            self.id = other.get_id().clone()
        }
        if self.datetime.is_empty() {
            self.datetime = other.get_datetime().to_string()
        }
        if self.data_blob.is_null() {
            self.data_blob = other.get_blob().clone()
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
