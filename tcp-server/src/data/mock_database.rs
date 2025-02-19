use crate::models::{Sensor, Session, SessionSensor, SessionSensorData, User};

use super::Database;

type Result<T> = crate::Result<T>;

pub struct MockDatabase;

impl MockDatabase {
    const USERNAME: &'static str = "username";
    const PASSWORD: &'static str = "password";
    const SESSION_ID: &'static str = "session_id";

    pub fn new() -> MockDatabase {
        MockDatabase {}
    }

    pub fn users() -> Vec<User> {
        vec![
            User::new(String::from("user_1"), String::from("pass_1")),
            User::new(String::from("user_2"), String::from("pass_2")),
            User::new(String::from("user_3"), String::from("pass_3")),
            User::new(String::from("user_4"), String::from("pass_4")),
        ]
    }

    pub fn sensors() -> Vec<Sensor> {
        vec![
            Sensor::new(String::from("id_1"), String::from("sensor_type_1")),
            Sensor::new(String::from("id_2"), String::from("sensor_type_2")),
            Sensor::new(String::from("id_3"), String::from("sensor_type_3")),
            Sensor::new(String::from("id_4"), String::from("sensor_type_4")),
        ]
    }

    pub fn sessions() -> Vec<Session> {
        vec![
            Session::new(String::from("session_id_1"), String::from("username_1")),
            Session::new(String::from("session_id_2"), String::from("username_2")),
            Session::new(String::from("session_id_3"), String::from("username_3")),
            Session::new(String::from("session_id_4"), String::from("username_4")),
        ]
    }

    pub fn sessions_sensors() -> Vec<SessionSensor> {
        vec![
            SessionSensor::new(
                String::from("session_sensor_id_1"),
                String::from("session_id_1"),
                String::from("sensor_id_1"),
            ),
            SessionSensor::new(
                String::from("session_sensor_id_2"),
                String::from("session_id_2"),
                String::from("sensor_id_2"),
            ),
            SessionSensor::new(
                String::from("session_sensor_id_3"),
                String::from("session_id_3"),
                String::from("sensor_id_3"),
            ),
            SessionSensor::new(
                String::from("session_sensor_id_4"),
                String::from("session_id_4"),
                String::from("sensor_id_4"),
            ),
        ]
    }

    pub fn sessions_sensors_data() -> Vec<SessionSensorData> {
        vec![
            SessionSensorData::new(
                String::from("id_1"),
                String::from("datetime_1"),
                String::from("data_blob_1"),
            ),
            SessionSensorData::new(
                String::from("id_2"),
                String::from("datetime_2"),
                String::from("data_blob_2"),
            ),
            SessionSensorData::new(
                String::from("id_3"),
                String::from("datetime_3"),
                String::from("data_blob_3"),
            ),
            SessionSensorData::new(
                String::from("id_4"),
                String::from("datetime_4"),
                String::from("data_blob_4"),
            ),
        ]
    }
}

impl Database for MockDatabase {
    fn get_session_user(&self, session_id: &str) -> Result<User> {
        Ok(User::new(
            session_id.to_string(),
            String::from(MockDatabase::PASSWORD),
        ))
    }

    fn is_admin(&self, _user: &User) -> bool {
        true
    }

    fn login(&self, _user: &User) -> Result<String> {
        Ok(String::from(MockDatabase::SESSION_ID))
    }

    fn logout(&self, _session_id: &str) -> Result<()> {
        Ok(())
    }

    fn renew_session(&self, _old_session: &str) -> Result<String> {
        Ok(format!("{}2", MockDatabase::SESSION_ID))
    }

    /* User */
    fn insert_user(&self, user: &User) -> Result<User> {
        Ok(user.clone())
    }

    fn get_users(&self) -> Result<Vec<User>> {
        Ok(MockDatabase::users())
    }

    fn get_user(&self, username: &str) -> Result<User> {
        Ok(User::new(String::from(username), String::new()))
    }

    /* Sensor */
    fn insert_sensor(&self, sensor: &Sensor) -> Result<Sensor> {
        Ok(Sensor::new(
            String::from(MockDatabase::SESSION_ID),
            sensor.get_sensor_type().to_string(),
        ))
    }

    fn get_sensors(&self) -> Result<Vec<Sensor>> {
        Ok(MockDatabase::sensors())
    }

    fn get_sensor(&self, sensor_id: &str) -> Result<Sensor> {
        Ok(Sensor::new(
            sensor_id.to_string(),
            String::from("sensor_type"),
        ))
    }

    /* Session */
    fn insert_session(&self, session: &Session) -> Result<Session> {
        Ok(Session::new(
            String::from(MockDatabase::SESSION_ID),
            session.get_username().to_string(),
        ))
    }

    fn get_session(&self, session_id: &str) -> Result<Session> {
        Ok(Session::new(
            session_id.to_string(),
            String::from(MockDatabase::USERNAME),
        ))
    }

    fn get_user_sessions(&self, username: &str) -> Result<Vec<Session>> {
        Ok(MockDatabase::sessions()
            .iter()
            .map(|session| Session::new(session.get_id().to_string(), username.to_string()))
            .collect::<Vec<_>>())
    }

    fn get_all_sessions(&self) -> Result<Vec<Session>> {
        Ok(MockDatabase::sessions())
    }

    /* Session Sensor */
    fn insert_session_sensor(&self, session_sensor: &SessionSensor) -> Result<SessionSensor> {
        Ok(SessionSensor::new(
            String::from("session_sensor_id"),
            session_sensor.get_session_id().to_string(),
            session_sensor.get_sensor_id().to_string(),
        ))
    }

    fn get_sessions_sensors(&self) -> Result<Vec<SessionSensor>> {
        Ok(MockDatabase::sessions_sensors())
    }

    fn get_session_sensors(&self, session_id: &str) -> Result<Vec<SessionSensor>> {
        Ok(MockDatabase::sessions_sensors()
            .iter()
            .map(|session_sensor| {
                SessionSensor::new(
                    session_sensor.get_id().to_string(),
                    session_id.to_string(),
                    session_sensor.get_sensor_id().to_string(),
                )
            })
            .collect::<Vec<_>>())
    }

    fn get_session_sensor(&self, session_sensor_id: &str) -> Result<SessionSensor> {
        Ok(SessionSensor::new(
            session_sensor_id.to_string(),
            String::from("session_id"),
            String::from("sensor_id"),
        ))
    }

    /* Session Sensor Data */
    fn insert_session_sensor_data(
        &self,
        session_sensor_data: &SessionSensorData,
    ) -> Result<SessionSensorData> {
        Ok(SessionSensorData::new(
            String::from("id"),
            session_sensor_data.get_datetime().to_string(),
            session_sensor_data.get_blob().to_string(),
        ))
    }

    fn batch_session_sensor_data(
        &self,
        data_blobs: &Vec<SessionSensorData>,
    ) -> Result<Vec<SessionSensorData>> {
        Ok(data_blobs
            .iter()
            .map(|blob| {
                SessionSensorData::new(
                    String::from("id"),
                    blob.get_datetime().to_string(),
                    blob.get_blob().to_string(),
                )
            })
            .collect::<Vec<_>>())
    }

    fn get_sessions_sensors_data(&self) -> Result<Vec<SessionSensorData>> {
        Ok(MockDatabase::sessions_sensors_data())
    }

    fn get_sessions_sensor_data(&self, _session_id: &str) -> Result<Vec<SessionSensorData>> {
        Ok(MockDatabase::sessions_sensors_data())
    }

    fn get_session_sensor_data(&self, session_sensor_id: &str) -> Result<Vec<SessionSensorData>> {
        Ok(MockDatabase::sessions_sensors_data()
            .iter()
            .map(|blob| {
                SessionSensorData::new(
                    session_sensor_id.to_string(),
                    blob.get_datetime().to_string(),
                    blob.get_blob().to_string(),
                )
            })
            .collect::<Vec<_>>())
    }

    fn get_session_sensor_datapoint(
        &self,
        session_sensor_id: &str,
        datetime: &str,
    ) -> Result<SessionSensorData> {
        Ok(SessionSensorData::new(
            session_sensor_id.to_string(),
            datetime.to_string(),
            String::from("data_blob"),
        ))
    }
}
