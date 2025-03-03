use rusqlite::{Connection, Result as RusqliteResult, params, Error as RusqliteError};
use crate::models::{User, Sensor, Session, SessionSensor, SessionSensorData};

use super::Database;
type Result<T> = crate::Result<T>;

#[allow(unused)]
pub struct SqliteDatabase {
    url: String,
    connection: Connection,
}

impl SqliteDatabase {
    #[allow(unused)]
    pub fn new(url: &str) -> Result<SqliteDatabase> {
        let connection = Connection::open(url).map_err(|e| e.to_string())?;
        Ok(SqliteDatabase {
            url: url.to_string(),
            connection,
        })
    }
}

//TODO: impl Database for SqliteDatabase {}
impl Database for SqliteDatabase {
    /* Authentication */
    fn get_session_user(&self, session_id: &str) -> Result<User> {
        let mut statement = self.connection.prepare(
            "SELECT User.username, User.password_hash
             FROM Session
             JOIN User ON Session.username = User.username
             WHERE Session.sessionID = ?1"
        ).map_err(|e| e.to_string())?;

        let user = statement.query_row(params![session_id], |row| {
            Ok(User::new(
                row.get(0)?,
                row.get(1)?
            ))
        }).map_err(|e| e.to_string())?;

        Ok(user)
    }

    // No column for admin
    fn is_admin(&self, user: &User) -> bool {
        unimplemented!()
    }

    fn login(&self, user: &User) -> Result<String> {
        unimplemented!()
    }

    fn logout(&self, session_id: &str) -> Result<()> {
        unimplemented!()
    }

    fn renew_session(&self, old_session: &str) -> Result<String> {
        unimplemented!()
    }

    /* User */
    // Does this pass in the hashed pw?
    fn insert_user(&self, user: &User) -> Result<User> {
        self.connection.execute(
            "INSERT INTO User (username, password_hash) VALUES (?1, ?2);", 
            params![user.get_username(), user.get_password_hash()]
        ).map_err(|e| e.to_string())?;

        // Returning the user for now
        Ok(user.clone())
    }
    
    fn get_users(&self) -> Result<Vec<User>> {
        let mut statement = self.connection.prepare(
            "SELECT username, password_hash FROM User"
        ).map_err(|e| e.to_string())?;

        let user_itr = statement.query_map([], |row| {
            Ok(User::new(
                row.get(0)?,
                row.get(1)?
            ))
        }).map_err(|e| e.to_string())?;

        let mut users = Vec::new();

        for user in user_itr {
            users.push(user.map_err(|e| e.to_string())?);
        }

        Ok(users)
    }
    
    // Just returns username and password_hash?
    fn get_user(&self, username: &str) -> Result<User> {
        let mut statement = self.connection.prepare(
            "SELECT username, password_hash FROM User WHERE username = ?1"
        ).map_err(|e| e.to_string())?;

        let user = statement.query_row(params![username], |row| {
            Ok(User::new(
                row.get(0)?,
                row.get(1)?
            ))
        }).map_err(|e| e.to_string())?;

        Ok(user)
    }

    /* Sensor */
    fn insert_sensor(&self, sensor: &Sensor) -> Result<Sensor> {
        unimplemented!()
    }
    
    fn get_sensors(&self) -> Result<Vec<Sensor>> {
        unimplemented!()
    }
    
    fn get_sensor(&self, sensor_id: &str) -> Result<Sensor> {
        unimplemented!()
    }

    /* Session */
    fn insert_session(&self, session: &Session) -> Result<Session> {
        unimplemented!()
    }
    
    fn get_session(&self, session_id: &str) -> Result<Session> {
        unimplemented!()
    }
    
    fn get_user_sessions(&self, username: &str) -> Result<Vec<Session>> {
        unimplemented!()
    }

    fn get_all_sessions(&self) -> Result<Vec<Session>> {
        unimplemented!()
    }

    /* Session Sensor */
    fn insert_session_sensor(&self, session_sensor: &SessionSensor) -> Result<SessionSensor> {
        unimplemented!()
    }
    
    fn get_sessions_sensors(&self) -> Result<Vec<SessionSensor>> {
        unimplemented!()
    }
    
    fn get_session_sensors(&self, session_id: &str) -> Result<Vec<SessionSensor>> {
        unimplemented!()
    }
    
    fn get_session_sensor(&self, session_sensor_id: &str) -> Result<SessionSensor> {
        unimplemented!()
    }

    /* Session Sensor Data */
    fn insert_session_sensor_data(
        &self,
        session_sensor_data: &SessionSensorData,
    ) -> Result<SessionSensorData> {
        unimplemented!()
    }

    fn batch_session_sensor_data(
        &self,
        data_blobs: &Vec<SessionSensorData>,
    ) -> Result<Vec<SessionSensorData>> {
        unimplemented!()
    }

    fn get_sessions_sensors_data(&self) -> Result<Vec<SessionSensorData>> {
        unimplemented!()
    }
    
    fn get_sessions_sensor_data(&self, session_id: &str) -> Result<Vec<SessionSensorData>> {
        unimplemented!()
    }
    
    fn get_session_sensor_data(&self, session_sensor_id: &str) -> Result<Vec<SessionSensorData>> {
        unimplemented!()
    }
    
    fn get_session_sensor_datapoint(
        &self,
        session_sensor_id: &str,
        datetime: &str,
    ) -> Result<SessionSensorData> {
        unimplemented!()
    }
}
