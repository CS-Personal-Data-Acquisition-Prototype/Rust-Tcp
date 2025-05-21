use serde_json::{Map, Value};

//#![cfg(not(feature = "sql"))]
use crate::models::{BaseModel, Sensor, Session, SessionSensor, SessionSensorData, User};

use super::Database;

type Result<T> = crate::Result<T>;

pub struct MockDatabase;

impl MockDatabase {
    const USERNAME: &'static str = "username";
    const PASSWORD: &'static str = "password";
    const COOKIE_SESSION_ID: &'static str = "cookie_session_id";
    const SESSION_ID: i64 = 12345;
    const SENSOR_ID: i64 = 67890;
    const SENSOR_TYPE: &'static str = "sensor_type";

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
            Sensor::new(1, String::from("sensor_type_1")),
            Sensor::new(2, String::from("sensor_type_2")),
            Sensor::new(3, String::from("sensor_type_3")),
            Sensor::new(4, String::from("sensor_type_4")),
        ]
    }

    pub fn sessions() -> Vec<Session> {
        vec![
            Session::new(1, String::from("username_1")),
            Session::new(2, String::from("username_2")),
            Session::new(3, String::from("username_3")),
            Session::new(4, String::from("username_4")),
        ]
    }

    pub fn sessions_sensors() -> Vec<SessionSensor> {
        vec![
            SessionSensor::new(1, 1, 1),
            SessionSensor::new(2, 2, 2),
            SessionSensor::new(3, 3, 3),
            SessionSensor::new(4, 4, 4),
        ]
    }

    pub fn sessions_sensors_data() -> Vec<SessionSensorData> {
        vec![
            SessionSensorData::new(Some(1), String::from("datetime_1"), Value::Object(Map::new())),
            SessionSensorData::new(Some(2), String::from("datetime_2"), Value::Object(Map::new())),
            SessionSensorData::new(Some(3), String::from("datetime_3"), Value::Object(Map::new())),
            SessionSensorData::new(Some(4), String::from("datetime_4"), Value::Object(Map::new())),
        ]
    }
}

impl Database for MockDatabase {
    fn temp_session_id_solution(&self) {}

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
        Ok(String::from(MockDatabase::COOKIE_SESSION_ID))
    }

    fn logout(&self, _session_id: &str) -> Result<()> {
        Ok(())
    }

    fn renew_session(&self, _old_session: &str) -> Result<String> {
        Ok(format!("{}2", MockDatabase::COOKIE_SESSION_ID))
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

    fn update_user(&self, username: &str, updated_user: &User) -> Result<User> {
        let mut user = User::empty();
        user.fill_from(updated_user);
        user.fill_from(&User::new(
            username.to_string(),
            MockDatabase::PASSWORD.to_string(),
        ));
        Ok(user)
    }

    fn delete_user(&self, _username: &str) -> Result<()> {
        Ok(())
    }

    /* Sensor */
    fn insert_sensor(&self, sensor: &Sensor) -> Result<Sensor> {
        Ok(Sensor::new(
            MockDatabase::SENSOR_ID,
            sensor.get_sensor_type().to_string(),
        ))
    }

    fn get_sensors(&self) -> Result<Vec<Sensor>> {
        Ok(MockDatabase::sensors())
    }

    fn get_sensor(&self, sensor_id: i64) -> Result<Sensor> {
        Ok(Sensor::new(sensor_id, String::from("sensor_type")))
    }

    fn update_sensor(&self, sensor_id: i64, updated_sensor: &Sensor) -> Result<Sensor> {
        let mut sensor = Sensor::empty();
        sensor.fill_from(updated_sensor);
        sensor.fill_from(&Sensor::new(
            sensor_id,
            MockDatabase::SENSOR_TYPE.to_string(),
        ));
        Ok(sensor)
    }

    fn delete_sensor(&self, _sensor_id: i64) -> Result<()> {
        Ok(())
    }

    /* Session */
    fn insert_session(&self, session: &Session) -> Result<Session> {
        Ok(Session::new(
            MockDatabase::SESSION_ID,
            session.get_username().to_string(),
        ))
    }

    fn get_session(&self, session_id: i64) -> Result<Session> {
        Ok(Session::new(
            session_id,
            String::from(MockDatabase::USERNAME),
        ))
    }

    fn get_user_sessions(&self, username: &str) -> Result<Vec<Session>> {
        Ok(MockDatabase::sessions()
            .iter()
            .map(|session| Session::new(session.get_id().clone(), username.to_string()))
            .collect::<Vec<_>>())
    }

    fn get_all_sessions(&self) -> Result<Vec<Session>> {
        Ok(MockDatabase::sessions())
    }

    fn update_session(&self, session_id: i64, updated_session: &Session) -> Result<Session> {
        let mut session = Session::empty();
        session.fill_from(updated_session);
        session.fill_from(&Session::new(
            session_id,
            MockDatabase::PASSWORD.to_string(),
        ));
        Ok(session)
    }

    fn delete_session(&self, _session_id: i64) -> Result<()> {
        Ok(())
    }

    /* Session Sensor */
    fn insert_session_sensor(&self, session_sensor: &SessionSensor) -> Result<SessionSensor> {
        Ok(SessionSensor::new(
            1,
            session_sensor.get_session_id().clone(),
            session_sensor.get_sensor_id().clone(),
        ))
    }

    fn get_sessions_sensors(&self) -> Result<Vec<SessionSensor>> {
        Ok(MockDatabase::sessions_sensors())
    }

    fn get_session_sensors(&self, session_id: i64) -> Result<Vec<SessionSensor>> {
        Ok(MockDatabase::sessions_sensors()
            .iter()
            .map(|session_sensor| {
                SessionSensor::new(
                    session_sensor.get_id().clone(),
                    session_id,
                    session_sensor.get_sensor_id().clone(),
                )
            })
            .collect::<Vec<_>>())
    }

    fn get_session_sensor(&self, session_sensor_id: i64) -> Result<SessionSensor> {
        Ok(SessionSensor::new(
            session_sensor_id,
            session_sensor_id + 1,
            session_sensor_id + 2,
        ))
    }

    fn update_session_sensor(
        &self,
        session_sensor_id: i64,
        updated_session_sensor: &SessionSensor,
    ) -> Result<SessionSensor> {
        let mut session_sensor = SessionSensor::empty();
        session_sensor.fill_from(updated_session_sensor);
        session_sensor.fill_from(&SessionSensor::new(
            session_sensor_id,
            MockDatabase::SESSION_ID,
            MockDatabase::SENSOR_ID,
        ));
        Ok(session_sensor)
    }

    fn delete_session_sensor(&self, _session_sensor_id: i64) -> Result<()> {
        Ok(())
    }

    /* Session Sensor Data */
    fn insert_session_sensor_data(
        &self,
        session_sensor_data: &SessionSensorData,
    ) -> Result<SessionSensorData> {
        Ok(SessionSensorData::new(
            Some(0),
            session_sensor_data.get_datetime().to_string(),
            session_sensor_data.get_blob().clone(),
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
                    Some(0),
                    blob.get_datetime().to_string(),
                    blob.get_blob().clone(),
                )
            })
            .collect::<Vec<_>>())
    }

    fn get_sessions_sensors_data(&self) -> Result<Vec<SessionSensorData>> {
        Ok(MockDatabase::sessions_sensors_data())
    }

    fn get_sessions_sensor_data(&self, _session_id: i64) -> Result<Vec<SessionSensorData>> {
        Ok(MockDatabase::sessions_sensors_data())
    }

    fn get_session_sensor_data(&self, session_sensor_id: i64) -> Result<Vec<SessionSensorData>> {
        Ok(MockDatabase::sessions_sensors_data()
            .iter()
            .map(|blob| {
                SessionSensorData::new(
                    Some(session_sensor_id),
                    blob.get_datetime().to_string(),
                    blob.get_blob().clone(),
                )
            })
            .collect())
    }

    fn get_session_sensor_datapoint(
        &self,
        session_id: i64,
        datetime: &str,
    ) -> Result<SessionSensorData> {
        Ok(SessionSensorData::new(
            Some(session_id),
            datetime.to_string(),
            Value::Object(Map::new()),
        ))
    }

    fn update_session_sensor_datapoint(
        &self,
        session_id: i64,
        datetime: &str,
        updated_session_sensor_datapoint: &SessionSensorData,
    ) -> Result<SessionSensorData> {
        let mut session_sensor_datapoint = SessionSensorData::empty();
        session_sensor_datapoint.fill_from(updated_session_sensor_datapoint);
        session_sensor_datapoint.fill_from(&SessionSensorData::new(
            Some(session_id),
            datetime.to_string(),
            Value::Object(Map::new()),
        ));
        Ok(session_sensor_datapoint)
    }

    fn delete_session_sensor_datapoint(
        &self,
        _session_id: i64,
        _datetime: &str,
    ) -> Result<()> {
        Ok(())
    }
}
