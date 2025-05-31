/*
Copyright 2025 CS 46X Personal Data Acquisition Prototype Group
    
Licensed under the Apache License, Version 2.0 (the "License"); you may not use this file except in compliance with the License.
You may obtain a copy of the License at http://www.apache.org/licenses/LICENSE-2.0
Unless required by applicable law or agreed to in writing, software distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied. See the License for the specific language governing permissions and limitations under the License.
*/

use crate::models::{Sensor, Session, SessionSensor, SessionSensorData, User};

type Result<T> = crate::Result<T>;

//TODO: add query parameters
pub trait Database {
    fn temp_session_id_solution(&self);
    /* Authentication */
    fn get_session_user(&self, session_id: &str) -> Result<User>;
    fn is_admin(&self, user: &User) -> bool;
    fn login(&self, user: &User) -> Result<String>;
    fn logout(&self, session_id: &str) -> Result<()>;
    fn renew_session(&self, old_session: &str) -> Result<String>;

    /* User */
    fn insert_user(&self, user: &User) -> Result<User>;
    fn get_users(&self) -> Result<Vec<User>>;
    fn get_user(&self, username: &str) -> Result<User>;
    fn update_user(&self, username: &str, updated_user: &User) -> Result<User>;
    fn delete_user(&self, username: &str) -> Result<()>;

    /* Sensor */
    fn insert_sensor(&self, sensor: &Sensor) -> Result<Sensor>;
    fn get_sensors(&self) -> Result<Vec<Sensor>>;
    fn get_sensor(&self, sensor_id: i64) -> Result<Sensor>;
    fn update_sensor(&self, sensor_id: i64, updated_sensor: &Sensor) -> Result<Sensor>;
    fn delete_sensor(&self, sensor_id: i64) -> Result<()>;

    /* Session */
    fn insert_session(&self, session: &Session) -> Result<Session>;
    fn get_session(&self, session_id: i64) -> Result<Session>;
    fn get_user_sessions(&self, username: &str) -> Result<Vec<Session>>;
    fn get_all_sessions(&self) -> Result<Vec<Session>>;
    fn update_session(&self, session_id: i64, updated_session: &Session) -> Result<Session>;
    fn delete_session(&self, session_id: i64) -> Result<()>;

    /* Session Sensor */
    fn insert_session_sensor(&self, session_sensor: &SessionSensor) -> Result<SessionSensor>;
    fn get_sessions_sensors(&self) -> Result<Vec<SessionSensor>>;
    fn get_session_sensors(&self, session_id: i64) -> Result<Vec<SessionSensor>>;
    fn get_session_sensor(&self, session_sensor_id: i64) -> Result<SessionSensor>;
    fn update_session_sensor(
        &self,
        session_sensor_id: i64,
        updated_session_sensor: &SessionSensor,
    ) -> Result<SessionSensor>;
    fn delete_session_sensor(&self, session_sensor_id: i64) -> Result<()>;

    /* Session Sensor Data */
    fn insert_session_sensor_data(
        &self,
        session_sensor_data: &SessionSensorData,
    ) -> Result<SessionSensorData>;
    fn batch_session_sensor_data(
        &self,
        data_blobs: &Vec<SessionSensorData>,
    ) -> Result<Vec<SessionSensorData>>;
    fn get_sessions_sensors_data(&self) -> Result<Vec<SessionSensorData>>;
    fn get_sessions_sensor_data(&self, session_id: i64) -> Result<Vec<SessionSensorData>>;
    fn get_sessions_sensor_data_after(
        &self,
        session_id: i64,
        datetime: &str,
    ) -> Result<Vec<SessionSensorData>>;
    fn get_session_sensor_data(&self, session_sensor_id: i64) -> Result<Vec<SessionSensorData>>;
    fn get_session_sensor_datapoint(
        &self,
        session_id: i64,
        datetime: &str,
    ) -> Result<SessionSensorData>;
    fn update_session_sensor_datapoint(
        &self,
        session_id: i64,
        datetime: &str,
        updated_session_sensor_datapoint: &SessionSensorData,
    ) -> Result<SessionSensorData>;
    fn delete_session_sensor_datapoint(&self, session_id: i64, datetime: &str) -> Result<()>;
}
