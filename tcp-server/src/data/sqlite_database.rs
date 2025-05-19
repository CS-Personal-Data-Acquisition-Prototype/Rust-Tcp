use crate::models::{Sensor, Session, SessionSensor, SessionSensorData, User};
use rusqlite::{params, Connection};

use super::Database;
type Result<T> = crate::Result<T>;

const SCHEMA_STATEMENT: &str = r#"
CREATE TABLE IF NOT EXISTS User (
    username TEXT PRIMARY KEY,
    password_hash TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS Session (
    sessionID INTEGER PRIMARY KEY,
    username TEXT NOT NULL,
    FOREIGN KEY (username) REFERENCES User(username) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS Sensor (
    sensorID INTEGER PRIMARY KEY,
    type TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS Session_Sensor (
    session_sensorID INTEGER PRIMARY KEY,
    sessionID INTEGER NOT NULL,
    sensorID INTEGER NOT NULL,
    FOREIGN KEY (sessionID) REFERENCES Session(sessionID) ON DELETE CASCADE,
    FOREIGN KEY (sensorID) REFERENCES Sensor(sensorID) ON DELETE CASCADE,
    UNIQUE (sessionID, sensorID)
);

CREATE TABLE IF NOT EXISTS Session_Sensor_Data (
    datetime TEXT,
    session_sensorID INTEGER,
    data_blob TEXT NOT NULL,
    PRIMARY KEY (datetime, session_sensorID),
    FOREIGN KEY (session_sensorID) REFERENCES Session_Sensor(session_sensorID) ON DELETE CASCADE
);
"#;

#[allow(unused)]
pub struct SqliteDatabase {
    url: String,
    connection: Connection,
    // Temp Mock database to replace the unimplemented functions for testing
    mock_db: super::MockDatabase,
}

#[allow(unused)]
impl SqliteDatabase {
    pub fn new(url: &str) -> Result<SqliteDatabase> {
        let connection = Connection::open(url).map_err(|e| e.to_string())?;
        Ok(SqliteDatabase {
            url: url.to_string(),
            connection,
            mock_db: super::MockDatabase::new(),
        })
    }

    // Constructor for use with testing functions
    pub fn from_connection(connection: Connection) -> Result<SqliteDatabase> {
        Ok(SqliteDatabase {
            url: ":memory:".to_string(),
            connection,
            mock_db: super::MockDatabase::new(),
        })
    }

    // Function to initialize the database schema
    pub fn init(&self) {
        if let Err(e) = self.connection.execute_batch(SCHEMA_STATEMENT) {
            panic!("Failed to create database schema: {e}")
        }
    }
}

//TODO: impl Database for SqliteDatabase {}
impl Database for SqliteDatabase {
    /* Authentication */
    // TODO: Implement
    // No column for admin
    fn is_admin(&self, user: &User) -> bool {
        self.mock_db.is_admin(user)
    }

    // TODO: Implement
    fn login(&self, user: &User) -> Result<String> {
        self.mock_db.login(user)
    }

    // TODO: Implement
    fn logout(&self, session_id: &str) -> Result<()> {
        self.mock_db.logout(session_id)
    }

    // TODO: Implement
    fn renew_session(&self, old_session: &str) -> Result<String> {
        self.mock_db.renew_session(old_session)
    }

    //TODO: Implement
    fn get_session_user(&self, session_id: &str) -> Result<User> {
        self.mock_db.get_session_user(session_id)
    }

    /* User */
    // Inserts a single User into User
    fn insert_user(&self, user: &User) -> Result<User> {
        self.connection
            .execute(
                "INSERT INTO User (username, password_hash) VALUES (?1, ?2);",
                params![user.get_username(), user.get_password_hash()],
            )
            .map_err(|e| e.to_string())?;

        // Returning the user for now
        Ok(user.clone())
    }

    // Returns all rows from User
    fn get_users(&self) -> Result<Vec<User>> {
        let mut statement = self
            .connection
            .prepare("SELECT username, password_hash FROM User")
            .map_err(|e| e.to_string())?;

        let user_itr = statement
            .query_map([], |row| Ok(User::new(row.get(0)?, row.get(1)?)))
            .map_err(|e| e.to_string())?;

        let mut user_vec = Vec::new();

        for user in user_itr {
            user_vec.push(user.map_err(|e| e.to_string())?);
        }

        Ok(user_vec)
    }

    // Returns a row from User where username matches
    fn get_user(&self, username: &str) -> Result<User> {
        let mut statement = self
            .connection
            .prepare("SELECT username, password_hash FROM User WHERE username = ?1")
            .map_err(|e| e.to_string())?;

        let user = statement
            .query_row(params![username], |row| {
                Ok(User::new(row.get(0)?, row.get(1)?))
            })
            .map_err(|e| e.to_string())?;

        Ok(user)
    }

    // Updates a user's information (note: username is the primary key and cannot be changed)
    fn update_user(&self, username: &str, updated_user: &User) -> Result<User> {
        let rows_updated = self
            .connection
            .execute(
                "UPDATE User SET password_hash = ?1 WHERE username = ?2",
                params![updated_user.get_password_hash(), username],
            )
            .map_err(|e| e.to_string())?;

        if rows_updated == 0 {
            return Err("Failed to update User".into());
        }

        Ok(updated_user.clone())
    }

    fn delete_user(&self, username: &str) -> Result<()> {
        let rows_updated = self
            .connection
            .execute("DELETE FROM User WHERE username = ?1", params![username])
            .map_err(|e| e.to_string())?;

        if rows_updated == 0 {
            return Err("Failed to delete from User".into());
        }

        Ok(())
    }

    /* Sensor */
    // Inserts a single Sensor into Sensor
    fn insert_sensor(&self, sensor: &Sensor) -> Result<Sensor> {
        self.connection
            .execute(
                "INSERT INTO Sensor (type) VALUES (?1)",
                params![sensor.get_sensor_type()],
            )
            .map_err(|e| e.to_string())?;
        Ok(Sensor::new(self.connection.last_insert_rowid(), sensor.get_sensor_type().to_string()))
    }

    // Returns all rows from Sensor
    fn get_sensors(&self) -> Result<Vec<Sensor>> {
        let mut statement = self
            .connection
            .prepare("SELECT sensorID, type FROM Sensor")
            .map_err(|e| e.to_string())?;

        let sensor_itr = statement
            .query_map([], |row| Ok(Sensor::new(row.get(0)?, row.get(1)?)))
            .map_err(|e| e.to_string())?;

        let mut sensor_vec = Vec::new();

        for sensor in sensor_itr {
            sensor_vec.push(sensor.map_err(|e| e.to_string())?);
        }

        Ok(sensor_vec)
    }

    // Returns a single row from Sensor where sensorID matches
    fn get_sensor(&self, sensor_id: i64) -> Result<Sensor> {
        let mut statement = self
            .connection
            .prepare("SELECT sensorID, type FROM Sensor WHERE sensorID = ?1")
            .map_err(|e| e.to_string())?;

        let sensor = statement
            .query_row(params![sensor_id], |row| {
                Ok(Sensor::new(row.get(0)?, row.get(1)?))
            })
            .map_err(|e| e.to_string())?;

        Ok(sensor)
    }

    fn update_sensor(&self, sensor_id: i64, updated_sensor: &Sensor) -> Result<Sensor> {
        let rows_updated = self
            .connection
            .execute(
                "UPDATE Sensor SET type = ?1 WHERE sensorID = ?2",
                params![updated_sensor.get_sensor_type(), sensor_id],
            )
            .map_err(|e| e.to_string())?;

        if rows_updated == 0 {
            return Err("Failed to update Sensor".into());
        }

        Ok(updated_sensor.clone())
    }

    fn delete_sensor(&self, sensor_id: i64) -> Result<()> {
        let rows_updated = self
            .connection
            .execute("DELETE FROM Sensor WHERE sensorID = ?1", params![sensor_id])
            .map_err(|e| e.to_string())?;

        if rows_updated == 0 {
            return Err("Failed to delete from Sensor".into());
        }

        Ok(())
    }

    /* Session */
    // Inserts a single Session into Session
    fn insert_session(&self, session: &Session) -> Result<Session> {
        self.connection
            .execute(
                "INSERT INTO Session (username) VALUES (?1)",
                params![session.get_username()],
            )
            .map_err(|e| e.to_string())?;

        Ok(Session::new(self.connection.last_insert_rowid(), session.get_username().to_string()))
    }

    // Returns a single row from Session where sessionID matches
    fn get_session(&self, session_id: i64) -> Result<Session> {
        let mut statement = self
            .connection
            .prepare("SELECT sessionID, username FROM Session WHERE sessionID = ?1")
            .map_err(|e| e.to_string())?;

        let session = statement
            .query_row(params![session_id], |row| {
                Ok(Session::new(row.get(0)?, row.get(1)?))
            })
            .map_err(|e| e.to_string())?;

        Ok(session)
    }

    // Returns rows from Session where username matches
    fn get_user_sessions(&self, username: &str) -> Result<Vec<Session>> {
        let mut statement = self
            .connection
            .prepare("SELECT sessionID, username FROM Session WHERE username = ?1")
            .map_err(|e| e.to_string())?;

        let session_itr = statement
            .query_map(params![username], |row| {
                Ok(Session::new(row.get(0)?, row.get(1)?))
            })
            .map_err(|e| e.to_string())?;

        let mut session_vec = Vec::new();

        for session in session_itr {
            session_vec.push(session.map_err(|e| e.to_string())?);
        }

        Ok(session_vec)
    }

    // Returns all rows from Session
    fn get_all_sessions(&self) -> Result<Vec<Session>> {
        let mut statement = self
            .connection
            .prepare("SELECT sessionID, username FROM Session")
            .map_err(|e| e.to_string())?;

        let session_itr = statement
            .query_map([], |row| Ok(Session::new(row.get(0)?, row.get(1)?)))
            .map_err(|e| e.to_string())?;

        let mut session_vec = Vec::new();

        for session in session_itr {
            session_vec.push(session.map_err(|e| e.to_string())?);
        }

        Ok(session_vec)
    }

    fn update_session(&self, session_id: i64, updated_session: &Session) -> Result<Session> {
        let rows_updated = self
            .connection
            .execute(
                "UPDATE Session SET username = ?1 WHERE sessionID = ?2",
                params![updated_session.get_username(), session_id],
            )
            .map_err(|e| e.to_string())?;

        if rows_updated == 0 {
            return Err("Failed to update Session".into());
        }

        Ok(updated_session.clone())
    }

    fn delete_session(&self, session_id: i64) -> Result<()> {
        let rows_updated = self
            .connection
            .execute(
                "DELETE FROM Session WHERE sessionID = ?1",
                params![session_id],
            )
            .map_err(|e| e.to_string())?;

        if rows_updated == 0 {
            return Err("Failed to delete from Session".into());
        }

        Ok(())
    }

    /* Session Sensor */
    // Inserts a single SessionSensor into Session_Sensor
    fn insert_session_sensor(&self, session_sensor: &SessionSensor) -> Result<SessionSensor> {
        self.connection
            .execute(
                "INSERT INTO Session_Sensor (sessionID, sensorID) VALUES (?1, ?2)",
                params![session_sensor.get_session_id(), session_sensor.get_sensor_id()]
            ).map_err(|e| e.to_string())?;

        Ok(SessionSensor::new(self.connection.last_insert_rowid(), *session_sensor.get_session_id(), *session_sensor.get_sensor_id()))
    }

    // Returns all rows from Session_Sensor
    fn get_sessions_sensors(&self) -> Result<Vec<SessionSensor>> {
        let mut statement = self
            .connection
            .prepare("SELECT session_sensorID, sessionID, sensorID FROM Session_Sensor")
            .map_err(|e| e.to_string())?;

        let session_sensor_itr = statement
            .query_map([], |row| {
                Ok(SessionSensor::new(row.get(0)?, row.get(1)?, row.get(2)?))
            })
            .map_err(|e| e.to_string())?;

        let mut session_sensor_vec = Vec::new();

        for session_sensor in session_sensor_itr {
            session_sensor_vec.push(session_sensor.map_err(|e| e.to_string())?);
        }

        Ok(session_sensor_vec)
    }

    // Returns rows from Session_Sensor where sessionID matches
    fn get_session_sensors(&self, session_id: i64) -> Result<Vec<SessionSensor>> {
        let mut statement = self
            .connection
            .prepare(
                "SELECT session_sensorID, sessionID, sensorID FROM Session_Sensor WHERE sessionID = ?1"
            ).map_err(|e| e.to_string())?;

        let session_sensor_itr = statement
            .query_map(params![session_id], |row| {
                Ok(SessionSensor::new(row.get(0)?, row.get(1)?, row.get(2)?))
            })
            .map_err(|e| e.to_string())?;

        let mut session_sensor_vec = Vec::new();

        for session_sensor in session_sensor_itr {
            session_sensor_vec.push(session_sensor.map_err(|e| e.to_string())?);
        }

        Ok(session_sensor_vec)
    }

    // Returns a single row from Session_Sensor where session_sensorID matches
    fn get_session_sensor(&self, session_sensor_id: i64) -> Result<SessionSensor> {
        let mut statement = self
            .connection
            .prepare(
                "SELECT session_sensorID, sessionID, sensorID FROM Session_Sensor WHERE session_sensorID = ?1"
            ).map_err(|e| e.to_string())?;

        let session_sensor = statement
            .query_row(params![session_sensor_id], |row| {
                Ok(SessionSensor::new(row.get(0)?, row.get(1)?, row.get(2)?))
            })
            .map_err(|e| e.to_string())?;

        Ok(session_sensor)
    }

    fn update_session_sensor(
        &self,
        session_sensor_id: i64,
        updated_session_sensor: &SessionSensor,
    ) -> Result<SessionSensor> {
        let rows_updated = self.connection
            .execute(
                "UPDATE Session_Sensor SET sessionID = ?1, sensorID = ?2 WHERE session_sensorID = ?3",
                params![updated_session_sensor.get_session_id(),
                    updated_session_sensor.get_sensor_id(),
                    session_sensor_id],
            )
            .map_err(|e| e.to_string())?;

        if rows_updated == 0 {
            return Err("Failed to update Session_Sensor".into());
        }

        Ok(updated_session_sensor.clone())
    }

    fn delete_session_sensor(&self, session_sensor_id: i64) -> Result<()> {
        let rows_updated = self
            .connection
            .execute(
                "DELETE FROM Session_Sensor WHERE session_sensorID = ?1",
                params![session_sensor_id],
            )
            .map_err(|e| e.to_string())?;

        if rows_updated == 0 {
            return Err("Failed to delete from Session_Sensor".into());
        }

        Ok(())
    }

    /* Session Sensor Data */
    // Inserts a single SessionSensorData into Session_Sensor_Data
    fn insert_session_sensor_data(
        &self,
        session_sensor_data: &SessionSensorData,
    ) -> Result<SessionSensorData> {
        self.connection
            .execute(
                "INSERT INTO Session_Sensor_Data (session_sensorID, datetime, data_blob) VALUES (?1, ?2, ?3)",
                params![session_sensor_data.get_id(), session_sensor_data.get_datetime(), session_sensor_data.get_blob()]
            ).map_err(|e| e.to_string())?;

        Ok(session_sensor_data.clone())
    }

    // Batch inserts data points
    fn batch_session_sensor_data(
        &self,
        data_blobs: &Vec<SessionSensorData>,
    ) -> Result<Vec<SessionSensorData>> {
        self.connection
            .execute_batch("BEGIN TRANSACTION;")
            .map_err(|e| e.to_string())?;

        for data in data_blobs {
            self.connection
                .execute(
                    "INSERT INTO Session_Sensor_Data (session_sensorID, datetime, data_blob) VALUES (?1, ?2, ?3)",
                    params![data.get_id(), data.get_datetime(), data.get_blob()]
                ).map_err(|e| e.to_string())?;
        }

        self.connection
            .execute_batch("COMMIT;")
            .map_err(|e| e.to_string())?;

        Ok(data_blobs.clone())
    }

    // Returns all rows from Session_Sensor_Data
    fn get_sessions_sensors_data(&self) -> Result<Vec<SessionSensorData>> {
        let mut statement = self
            .connection
            .prepare("SELECT session_sensorID, datetime, data_blob FROM Session_Sensor_Data")
            .map_err(|e| e.to_string())?;

        let session_sensor_data_itr = statement
            .query_map([], |row| {
                let id: i64 = row.get(0)?;
                let datetime: String = row.get(1)?;
                let data_blob: String = row.get(2)?;
                Ok(SessionSensorData::new(id, datetime, data_blob))
            })
            .map_err(|e| e.to_string())?;

        let mut session_sensor_data_vec = Vec::new();
        for session_sensor_data in session_sensor_data_itr {
            session_sensor_data_vec.push(session_sensor_data.map_err(|e| e.to_string())?);
        }

        Ok(session_sensor_data_vec)
    }

    // Returns all rows from Session_Sensor_Data where session_sensorID is in Session_Sensor where sessionID matches
    fn get_sessions_sensor_data(&self, session_id: i64) -> Result<Vec<SessionSensorData>> {
        let mut statement = self
            .connection
            .prepare(
                "SELECT session_sensorID, datetime, data_blob FROM Session_Sensor_Data WHERE session_sensorID IN (SELECT session_sensorID FROM Session_Sensor WHERE sessionID = ?1)",
            )
            .map_err(|e| e.to_string())?;

        let session_sensor_data_itr = statement
            .query_map(params![session_id], |row| {
                let id: i64 = row.get(0)?;
                let datetime: String = row.get(1)?;
                let data_blob: String = row.get(2)?;
                Ok(SessionSensorData::new(id, datetime, data_blob))
            })
            .map_err(|e| e.to_string())?;

        let mut session_sensor_data_vec = Vec::new();
        for session_sensor_data in session_sensor_data_itr {
            session_sensor_data_vec.push(session_sensor_data.map_err(|e| e.to_string())?);
        }

        Ok(session_sensor_data_vec)
    }

    // Returns all rows from Session_Sensor_Data where session_sensorID matches
    fn get_session_sensor_data(&self, session_sensor_id: i64) -> Result<Vec<SessionSensorData>> {
        let mut statement = self
            .connection
            .prepare(
                "SELECT session_sensorID, datetime, data_blob FROM Session_Sensor_Data WHERE session_sensorID = ?1",
            )
            .map_err(|e| e.to_string())?;

        let session_sensor_data_itr = statement
            .query_map(params![session_sensor_id], |row| {
                let id: i64 = row.get(0)?;
                let datetime: String = row.get(1)?;
                let data_blob: String = row.get(2)?;
                Ok(SessionSensorData::new(id, datetime, data_blob))
            })
            .map_err(|e| e.to_string())?;

        let mut session_sensor_data_vec = Vec::new();
        for session_sensor_data in session_sensor_data_itr {
            session_sensor_data_vec.push(session_sensor_data.map_err(|e| e.to_string())?);
        }

        Ok(session_sensor_data_vec)
    }

    // Returns a single datapoint that matches a session_sensor_id and datetime
    fn get_session_sensor_datapoint(
        &self,
        session_sensor_id: i64,
        datetime: &str,
    ) -> Result<SessionSensorData> {
        let mut statement = self
            .connection
            .prepare(
                "SELECT session_sensorID, datetime, data_blob FROM Session_Sensor_Data 
             WHERE session_sensorID = ?1
             AND datetime = ?2",
            )
            .map_err(|e| e.to_string())?;

        let session_sensor_datapoint = statement
            .query_row(params![session_sensor_id, datetime], |row| {
                let id: i64 = row.get(0)?;
                let datetime: String = row.get(1)?;
                let data_blob: String = row.get(2)?;

                Ok(SessionSensorData::new(id, datetime, data_blob))
            })
            .map_err(|e| e.to_string())?;

        Ok(session_sensor_datapoint)
    }

    fn update_session_sensor_datapoint(
        &self,
        session_sensor_id: i64,
        datetime: &str,
        updated_session_sensor_datapoint: &SessionSensorData,
    ) -> Result<SessionSensorData> {
        let rows_updated = self.connection
            .execute(
                "UPDATE Session_Sensor_Data SET data_blob = ?1 WHERE datetime = ?2 AND session_sensorID = ?3",
                params![updated_session_sensor_datapoint.get_blob(), datetime, session_sensor_id],
            )
            .map_err(|e| e.to_string())?;

        if rows_updated == 0 {
            return Err("Failed to update Session_Sensor_Data".into());
        }

        Ok(updated_session_sensor_datapoint.clone())
    }

    fn delete_session_sensor_datapoint(
        &self,
        session_sensor_id: i64,
        datetime: &str,
    ) -> Result<()> {
        let rows_updated = self
            .connection
            .execute(
                "DELETE FROM Session_Sensor_Data WHERE session_sensorID = ?1 AND datetime = ?2",
                params![session_sensor_id, datetime],
            )
            .map_err(|e| e.to_string())?;

        if rows_updated == 0 {
            return Err("Failed to delete from Session_Sensor_Data".into());
        }

        Ok(())
    }
}
