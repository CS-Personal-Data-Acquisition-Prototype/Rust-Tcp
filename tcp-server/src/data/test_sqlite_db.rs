#[cfg(test)]
mod tests {
    use rusqlite::{Connection, Result as RusqliteResult, params};
    use crate::models::{BaseModel, Sensor, Session, SessionSensor, SessionSensorData, User};
    use crate::data::{Database, SqliteDatabase};

    /* Helpers */

    macro_rules! assert_models_eq {
        ($expected:expr, $actual:expr, [ $( $field:ident ),* ]) => {
            $(
                assert_eq!(
                    $expected.$field(),
                    $actual.$field(),
                    concat!("Mismatch on field: ", stringify!($field))
                );
            )*
        };
    }


    fn init_schema() -> Connection {
        let conn = Connection::open_in_memory().expect("Failed to open connection in memory");

        conn
            .execute_batch(
                r#"
                CREATE TABLE IF NOT EXISTS User (
                    username TEXT PRIMARY KEY,
                    password_hash TEXT NOT NULL
                );

                CREATE TABLE IF NOT EXISTS Session (
                    sessionID TEXT PRIMARY KEY,
                    username TEXT NOT NULL,
                    FOREIGN KEY (username) REFERENCES User(username) ON DELETE CASCADE
                );

                CREATE TABLE IF NOT EXISTS Sensor (
                    sensorID TEXT PRIMARY KEY,
                    type TEXT NOT NULL
                );

                CREATE TABLE IF NOT EXISTS Session_Sensor (
                    session_sensorID TEXT PRIMARY KEY,
                    sessionID TEXT NOT NULL,
                    sensorID TEXT NOT NULL,
                    FOREIGN KEY (sessionID) REFERENCES Session(sessionID) ON DELETE CASCADE,
                    FOREIGN KEY (sensorID) REFERENCES Sensor(sensorID) ON DELETE CASCADE
                );

                CREATE TABLE IF NOT EXISTS Session_Sensor_Data (
                    datetime TEXT,
                    session_sensorID TEXT,
                    data_blob BLOB NOT NULL,
                    PRIMARY KEY (datetime, session_sensorID),
                    FOREIGN KEY (session_sensorID) REFERENCES Session_Sensor(session_sensorID) ON DELETE CASCADE
                );
            "#,
            )
            .expect("Failed to initialize schema");

            conn
    }

    fn add_test_user(conn: &Connection, user: &User) {
        conn
            .execute(
                "INSERT INTO User (username, password_hash) VALUES (?1, ?2)",
                params![user.get_username(), user.get_password_hash()],
            ).expect("Failed to insert test user");
    }

    fn add_test_session(conn: &Connection, session: &Session) {
        conn
            .execute(
                "INSERT INTO Session (sessionID, username) VALUES (?1, ?2)",
                params![session.get_id(), session.get_username()],
            ).expect("Failed to insert test session");
    }

    fn add_test_sensor(conn: &Connection, sensor: &Sensor) {
        conn
            .execute(
                "INSERT INTO Sensor (sensorID, type) VALUES (?1, ?2)",
                params![sensor.get_id(), sensor.get_sensor_type()],
            ).expect("Failed to insert test sensor");
    }
    
    fn add_test_session_sensor(conn: &Connection, session_sensor: &SessionSensor) {
        conn
            .execute(
                "INSERT INTO Session_Sensor (session_sensorID, sessionID, sensorID) VALUES (?1, ?2, ?3)",
                params![session_sensor.get_id(), session_sensor.get_session_id(), session_sensor.get_sensor_id()]
            ).expect("Failed to insert test session_sensor");
    }

    fn add_test_session_sensor_data(conn: &Connection, session_sensor_data: &SessionSensorData) {
        conn
            .execute(
                "INSERT INTO Session_Sensor_Data (session_sensorID, datetime, data_blob) VALUES (?1, ?2, ?3)",
                params![session_sensor_data.get_id(), session_sensor_data.get_datetime(), session_sensor_data.get_blob().to_string()]
            ).expect("Failed to insert test session_sensor_data");
    }

    /* Tests */

 
    /*
    #[test]
    fn is_admin() {
    }

    #[test]
    fn login() {
    }

    #[test]
    fn logout() {
    }

    #[test]
    fn renew_session() {
    }
    */

    #[test]
    fn test_insert_user() {
        let conn = init_schema();

        let db = SqliteDatabase::from_connection(conn).expect("Failed to create test db");

        let user = User::new("NewUser".to_string(), "pwordHashed".to_string());

        let result = db.insert_user(&user);
        assert!(result.is_ok());

        // Check db for new user
        let fetch_user_result = db.get_user(user.get_username());
        assert!(fetch_user_result.is_ok());

        let fetched_user = fetch_user_result.unwrap();
        assert_models_eq!(user, fetched_user, [get_username, get_password_hash]);
    }

    #[test]
    fn test_get_users() {
        let conn = init_schema();
        
        let users_to_add = vec![
            User::new("user1".to_string(), "pwordHashed".to_string()),
            User::new("user2".to_string(), "my_password123".to_string()),
            User::new("user3".to_string(), "hunter2".to_string()),
        ];

        for user in &users_to_add {
            add_test_user(&conn, user);
        }

        let db = SqliteDatabase::from_connection(conn).expect("Failed to create test db");

        let result = db.get_users();
        assert!(result.is_ok());

        let users = result.unwrap();
        assert_eq!(users.len(), users_to_add.len());

        for test_user in &users_to_add {
            let matched = users.iter().find(|u|
                u.get_username() == test_user.get_username()  &&
                u.get_password_hash() == test_user.get_password_hash()
            );

            assert!(matched.is_some(), "User {:?} not found in db result", test_user.get_username());
        }
    }

    #[test]
    fn test_get_user() {
        let conn = init_schema();
        
        let user = User::new("NewUser".to_string(), "pwordHashed".to_string());
        add_test_user(&conn, &user);

        let db = SqliteDatabase::from_connection(conn).expect("Failed to create test db");

        let result = db.get_user(user.get_username());
        assert!(result.is_ok());

        let returned_user = result.unwrap();
        assert_models_eq!(user, returned_user, [get_username, get_password_hash]);
    }

    #[test]
    fn test_update_user() {
        let conn = init_schema();
        
        let original_user = User::new("TestUser".to_string(), "pwordHashed".to_string());
        add_test_user(&conn, &original_user);

        let db = SqliteDatabase::from_connection(conn).expect("Failed to create test db");
        
        // Can't change username, it's the primary key
        let updated_user = User::new(original_user.get_username().to_string(), "new_password".to_string());

        let result = db.update_user(original_user.get_username(), &updated_user);
        assert!(result.is_ok());

        // Verify the user has been updated
        let fetch_updated_result = db.get_user(updated_user.get_username());
        assert!(fetch_updated_result.is_ok());

        let fetched_user = fetch_updated_result.unwrap();
        assert_models_eq!(updated_user, fetched_user, [get_username, get_password_hash]);
    }

    #[test]
    fn test_delete_user() {
        let conn = init_schema();

        let user = User::new("TestUser".to_string(), "pwordHashed".to_string());
        add_test_user(&conn, &user);

        let db = SqliteDatabase::from_connection(conn).expect("Failed to create test db");

        let result = db.delete_user(user.get_username());
        assert!(result.is_ok());

        // Verify that the user does not exist in the db
        let fetch_user_result = db.get_user(user.get_username());
        assert!(fetch_user_result.is_err());
    }

    #[test]
    fn test_insert_sensor() {
        let conn = init_schema();

        let db = SqliteDatabase::from_connection(conn).expect("Failed to create test db");

        let sensor = Sensor::new("NewSensor".to_string(), "Acceleration".to_string());

        let result = db.insert_sensor(&sensor);
        assert!(result.is_ok());

        // Check db for new sensor
        let fetch_sensor_result = db.get_sensor(sensor.get_id());
        assert!(fetch_sensor_result.is_ok());

        let fetched_sensor = fetch_sensor_result.unwrap();
        assert_models_eq!(sensor, fetched_sensor, [get_id, get_sensor_type]);
    }

    #[test]
    fn test_get_sensors() {
        let conn = init_schema();
        
        let sensors_to_add = vec![
            Sensor::new("sensor1".to_string(), "Acceleration".to_string()),
            Sensor::new("sensor2".to_string(), "GPS".to_string()),
            Sensor::new("sensor3".to_string(), "Altitude".to_string()),
        ];

        for sensor in &sensors_to_add { 
            add_test_sensor(&conn, sensor);
        }

        let db = SqliteDatabase::from_connection(conn).expect("Failed to create test db");

        let result = db.get_sensors();
        assert!(result.is_ok());

        let sensors = result.unwrap();
        assert_eq!(sensors.len(), sensors_to_add.len());

        for test_sensor in &sensors_to_add {
            let matched = sensors.iter().find(|s|
                s.get_id() == test_sensor.get_id()  &&
                s.get_sensor_type() == test_sensor.get_sensor_type()
            );

            assert!(matched.is_some(), "Sensor {:?} not found in db result", test_sensor.get_id());
        }
    }

    #[test]
    fn test_get_sensor() {
        let conn = init_schema();

        let sensor = Sensor::new("NewSensor".to_string(), "Acceleration".to_string());
        add_test_sensor(&conn, &sensor);

        let db = SqliteDatabase::from_connection(conn).expect("Failed to create test db");

        let result = db.get_sensor(sensor.get_id());
        assert!(result.is_ok());

        let returned_sensor = result.unwrap();
        assert_models_eq!(sensor, returned_sensor, [get_id, get_sensor_type]);
    }

    #[test]
    fn test_update_sensor() {
        let conn = init_schema();
        
        let original_sensor = Sensor::new("TestSensor".to_string(), "Acceleration".to_string());
        add_test_sensor(&conn, &original_sensor);

        let db = SqliteDatabase::from_connection(conn).expect("Failed to create test db");
        
        // Can't change id, it's the primary key
        let updated_sensor = Sensor::new(original_sensor.get_id().to_string(), "GPS".to_string());

        let result = db.update_sensor(original_sensor.get_id(), &updated_sensor);
        assert!(result.is_ok());

        // Verify the user has been updated
        let fetch_updated_result = db.get_sensor(updated_sensor.get_id());
        assert!(fetch_updated_result.is_ok());

        let fetched_sensor = fetch_updated_result.unwrap();
        assert_models_eq!(updated_sensor, fetched_sensor, [get_id, get_sensor_type]);
    }

    #[test]
    fn test_delete_sensor() {
        let conn = init_schema();

        let sensor = Sensor::new("TestSensor".to_string(), "Acceleration".to_string());
        add_test_sensor(&conn, &sensor);

        let db = SqliteDatabase::from_connection(conn).expect("Failed to create test db");

        let result = db.delete_sensor(sensor.get_id());
        assert!(result.is_ok());

        // Verify that the user does not exist in the db
        let fetch_sensor_result = db.get_sensor(sensor.get_id());
        assert!(fetch_sensor_result.is_err());
    }

    #[test]
    fn test_insert_session() {
        let conn = init_schema();

        let user = User::new("TestUser".to_string(), "pwordHashed".to_string());
        add_test_user(&conn, &user);

        let db = SqliteDatabase::from_connection(conn).expect("Failed to create test db");
        
        let session = Session::new("TestSession".to_string(), "TestUser".to_string());

        let result = db.insert_session(&session);
        assert!(result.is_ok());

        // Check db for new session 
        let fetch_session_result = db.get_session(session.get_id());
        assert!(fetch_session_result.is_ok());

        let fetched_session = fetch_session_result.unwrap();
        assert_models_eq!(session, fetched_session, [get_id, get_username]);
    }
    
    #[test]
    fn test_get_session() {
        let conn = init_schema();
        
        let user = User::new("TestUser".to_string(), "pwordHashed".to_string());
        add_test_user(&conn, &user);

        let session = Session::new("TestSession".to_string(), "TestUser".to_string());
        add_test_session(&conn, &session);

        let db = SqliteDatabase::from_connection(conn).expect("Failed to create test db");

        let result = db.get_session(session.get_id());
        assert!(result.is_ok());

        let returned_session = result.unwrap();
        assert_models_eq!(session, returned_session, [get_id, get_username]);
    }

    #[test]
    fn test_get_session_user() {
        let conn = init_schema();

        let user = User::new("TestUser".to_string(), "pwordHashed".to_string());
        add_test_user(&conn, &user);

        let session = Session::new("session123".to_string(), "TestUser".to_string());
        add_test_session(&conn, &session);

        let db = SqliteDatabase::from_connection(conn).expect("Failed to create test db");

        let result = db.get_session_user(session.get_id());
        assert!(result.is_ok());

        let returned_user = result.unwrap();
        assert_models_eq!(user, returned_user, [get_username, get_password_hash]);
    }


    #[test]
    fn test_get_user_sessions() {
        let conn = init_schema();

        let user = User::new("TestUser".to_string(), "pwordHashed".to_string());
        add_test_user(&conn, &user);

        let sessions_to_add = vec![
            Session::new("session1".to_string(), "TestUser".to_string()),
            Session::new("session2".to_string(), "TestUser".to_string()),
            Session::new("session3".to_string(), "TestUser".to_string()),
        ];

        for session in &sessions_to_add { 
            add_test_session(&conn, session);
        }

        let db = SqliteDatabase::from_connection(conn).expect("Failed to create test db");

        let result = db.get_user_sessions(user.get_username());
        assert!(result.is_ok());

        let sessions = result.unwrap();
        assert_eq!(sessions.len(), sessions_to_add.len());

        for test_session in &sessions_to_add {
            let matched = sessions.iter().find(|s|
                s.get_id() == test_session.get_id()  &&
                s.get_username() == test_session.get_username()
            );

            assert!(matched.is_some(), "Session {:?} not found in db result", test_session.get_id());
        }
    }

    #[test]
    fn test_get_all_sessions() {
        let conn = init_schema();

        let user1 = User::new("TestUser".to_string(), "pwordHashed".to_string());
        add_test_user(&conn, &user1);

        let user2 = User::new("other_user".to_string(), "my_password123".to_string());
        add_test_user(&conn, &user2);

        let sessions_to_add = vec![
            Session::new("session1".to_string(), "TestUser".to_string()),
            Session::new("session2".to_string(), "TestUser".to_string()),
            Session::new("session3".to_string(), "TestUser".to_string()),
            Session::new("session4".to_string(), "other_user".to_string()),
            Session::new("session5".to_string(), "other_user".to_string()),
        ];

        for session in &sessions_to_add { 
            add_test_session(&conn, session);
        }

        let db = SqliteDatabase::from_connection(conn).expect("Failed to create test db");

        let result = db.get_all_sessions();
        assert!(result.is_ok());

        let sessions = result.unwrap();
        assert_eq!(sessions.len(), sessions_to_add.len());

        for test_session in &sessions_to_add {
            let matched = sessions.iter().find(|s|
                s.get_id() == test_session.get_id()  &&
                s.get_username() == test_session.get_username()
            );

            assert!(matched.is_some(), "Session {:?} not found in db result", test_session.get_id());
        }
    }
    
    #[test]
    fn test_update_session() {
        let conn = init_schema();

        let user1 = User::new("TestUser".to_string(), "pwordHashed".to_string());
        add_test_user(&conn, &user1);

        let user2 = User::new("other_user".to_string(), "my_password123".to_string());
        add_test_user(&conn, &user2);
        
        let original_session = Session::new("session1".to_string(), "TestUser".to_string());
        add_test_session(&conn, &original_session);

        let db = SqliteDatabase::from_connection(conn).expect("Failed to create test db");
        
        // Can't change id, it's the primary key
        let updated_session = Session::new(original_session.get_id().to_string(), "other_user".to_string());

        let result = db.update_session(original_session.get_id(), &updated_session);
        assert!(result.is_ok());

        // Verify the user has been updated
        let fetch_updated_result = db.get_session(updated_session.get_id());
        assert!(fetch_updated_result.is_ok());

        let fetched_session = fetch_updated_result.unwrap();
        assert_models_eq!(updated_session, fetched_session, [get_id, get_username]);
    }

    #[test]
    fn test_delete_session() {
        let conn = init_schema();

        let user = User::new("TestUser".to_string(), "pwordHashed".to_string());
        add_test_user(&conn, &user);

        let session = Session::new("session1".to_string(), "TestUser".to_string());
        add_test_session(&conn, &session);

        let db = SqliteDatabase::from_connection(conn).expect("Failed to create test db");

        let result = db.delete_session(session.get_id());
        assert!(result.is_ok());

        // Verify that the user does not exist in the db
        let fetch_session_result = db.get_session(session.get_id());
        assert!(fetch_session_result.is_err());
    }

    #[test]
    fn test_insert_session_sensor() {
        let conn = init_schema();

        let user = User::new("user1".to_string(), "hunter2".to_string());
        add_test_user(&conn, &user);

        let session = Session::new("session1".to_string(), user.get_username().to_string());
        add_test_session(&conn, &session);

        let sensor = Sensor::new("sensor1".to_string(), "Acceleration".to_string());
        add_test_sensor(&conn, &sensor);

        let db = SqliteDatabase::from_connection(conn).expect("Failed to create test db");

        let session_sensor = SessionSensor::new("session_sensor1".to_string(), 
            session.get_id().to_string(), sensor.get_id().to_string());

        let result = db.insert_session_sensor(&session_sensor);
        assert!(result.is_ok());

        // Check db for new session_sensor
        let fetch_session_sensor_result = db.get_session_sensor(session_sensor.get_id());
        assert!(fetch_session_sensor_result.is_ok());

        let fetched_session_sensor = fetch_session_sensor_result.unwrap();
        assert_models_eq!(session_sensor, fetched_session_sensor, [get_id, get_session_id, get_sensor_id]);
    }

    #[test]
    fn test_get_sessions_sensors() {
        let conn = init_schema();

        let user = User::new("user1".to_string(), "hunter2".to_string());
        add_test_user(&conn, &user);

        let session = Session::new("session1".to_string(), user.get_username().to_string());
        add_test_session(&conn, &session);

        let sensor = Sensor::new("sensor1".to_string(), "Acceleration".to_string());
        add_test_sensor(&conn, &sensor);

        let sessions_sensors_to_add = vec![
            SessionSensor::new("session_sensor1".to_string(), 
                session.get_id().to_string(), sensor.get_id().to_string()),

            SessionSensor::new("session_sensor2".to_string(), 
                session.get_id().to_string(), sensor.get_id().to_string()),

            SessionSensor::new("session_sensor3".to_string(), 
                session.get_id().to_string(), sensor.get_id().to_string()),
        ];

        for session_sensor in &sessions_sensors_to_add { 
            add_test_session_sensor(&conn, session_sensor);
        }

        let db = SqliteDatabase::from_connection(conn).expect("Failed to create test db");

        let result = db.get_sessions_sensors();
        assert!(result.is_ok());

        let sessions_sensors = result.unwrap();
        assert_eq!(sessions_sensors.len(), sessions_sensors_to_add.len());

        for test_session_sensor in &sessions_sensors_to_add {
            let matched = sessions_sensors.iter().find(|s|
                s.get_id() == test_session_sensor.get_id()  &&
                s.get_session_id() == test_session_sensor.get_session_id()  &&
                s.get_sensor_id() == test_session_sensor.get_sensor_id()
            );

            assert!(matched.is_some(), "SessionSensor {:?} not found in db result", test_session_sensor.get_id());
        }
    }

    #[test]
    fn test_get_session_sensors() {
        let conn = init_schema();

        let user = User::new("user1".to_string(), "hunter2".to_string());
        add_test_user(&conn, &user);

        let session = Session::new("session1".to_string(), user.get_username().to_string());
        add_test_session(&conn, &session);

        let sensor = Sensor::new("sensor1".to_string(), "Acceleration".to_string());
        add_test_sensor(&conn, &sensor);

        let sessions_sensors_to_add = vec![
            SessionSensor::new("session_sensor1".to_string(), 
                session.get_id().to_string(), sensor.get_id().to_string()),

            SessionSensor::new("session_sensor2".to_string(), 
                session.get_id().to_string(), sensor.get_id().to_string()),

            SessionSensor::new("session_sensor3".to_string(), 
                session.get_id().to_string(), sensor.get_id().to_string()),
        ];

        for session_sensor in &sessions_sensors_to_add { 
            add_test_session_sensor(&conn, session_sensor);
        }

        let db = SqliteDatabase::from_connection(conn).expect("Failed to create test db");

        let result = db.get_session_sensors(session.get_id());
        assert!(result.is_ok());

        let sessions_sensors = result.unwrap();
        assert_eq!(sessions_sensors.len(), sessions_sensors_to_add.len());

        for test_session_sensor in &sessions_sensors_to_add {
            let matched = sessions_sensors.iter().find(|s|
                s.get_id() == test_session_sensor.get_id()  &&
                s.get_session_id() == test_session_sensor.get_session_id()  &&
                s.get_sensor_id() == test_session_sensor.get_sensor_id()
            );

            assert!(matched.is_some(), "SessionSensor {:?} not found in db result", test_session_sensor.get_id());
        }
    }

    #[test]
    fn test_get_session_sensor() {
        let conn = init_schema();

        let user = User::new("user1".to_string(), "hunter2".to_string());
        add_test_user(&conn, &user);

        let session = Session::new("session1".to_string(), user.get_username().to_string());
        add_test_session(&conn, &session);

        let sensor = Sensor::new("sensor1".to_string(), "Acceleration".to_string());
        add_test_sensor(&conn, &sensor);

        let session_sensor = SessionSensor::new("session_sensor1".to_string(), 
            session.get_id().to_string(), sensor.get_id().to_string());
        add_test_session_sensor(&conn, &session_sensor);

        let db = SqliteDatabase::from_connection(conn).expect("Failed to create test db");

        let result = db.get_session_sensor(session_sensor.get_id());
        assert!(result.is_ok());

        let returned_session_sensor = result.unwrap();
        assert_models_eq!(session_sensor, returned_session_sensor, [get_id, get_session_id, get_sensor_id]);
    }

    #[test]
    fn test_update_session_sensor() {
        let conn = init_schema();
        
        let user = User::new("user1".to_string(), "hunter2".to_string());
        add_test_user(&conn, &user);

        let session1 = Session::new("session1".to_string(), user.get_username().to_string());
        add_test_session(&conn, &session1);

        let session2 = Session::new("session2".to_string(), user.get_username().to_string());
        add_test_session(&conn, &session2);

        let sensor1 = Sensor::new("sensor1".to_string(), "Acceleration".to_string());
        add_test_sensor(&conn, &sensor1);

        let sensor2 = Sensor::new("sensor2".to_string(), "GPS".to_string());
        add_test_sensor(&conn, &sensor2);

        let original_session_sensor = SessionSensor::new("session_sensor1".to_string(), 
            session1.get_id().to_string(), sensor1.get_id().to_string());
        add_test_session_sensor(&conn, &original_session_sensor);

        let db = SqliteDatabase::from_connection(conn).expect("Failed to create test db");

        let updated_session_sensor = SessionSensor::new(original_session_sensor.get_id().to_string(), 
            session2.get_id().to_string(), sensor2.get_id().to_string());

        let result = db.update_session_sensor(original_session_sensor.get_id(), &updated_session_sensor);
        assert!(result.is_ok());

        // Verify the session_sensor has been updated
        let fetch_updated_result = db.get_session_sensor(updated_session_sensor.get_id());
        assert!(fetch_updated_result.is_ok());

        let fetched_session_sensor = fetch_updated_result.unwrap();
        assert_models_eq!(updated_session_sensor, fetched_session_sensor, [get_id, get_session_id, get_sensor_id]);
    }

    /*
    #[test]
    fn test_insert_session_sensor_data() {
        let conn = init_schema();

        let user = User::new("user1".to_string(), "hunter2".to_string());
        add_test_user(&conn, &user);

        let session = Session::new("session1".to_string(), user.get_username().to_string());
        add_test_session(&conn, &session);

        let sensor = Sensor::new("sensor1".to_string(), "Acceleration".to_string());
        add_test_sensor(&conn, &sensor);

        let session_sensor = SessionSensor::new("session_sensor1".to_string(), 
            session.get_id().to_string(), sensor.get_id().to_string());
        add_test_session_sensor(&conn, &session_sensor);

        let db = SqliteDatabase::from_connection(conn).expect("Failed to create test db");

        let session_sensor_data = SessionSensorData::new(session_sensor.get_id().to_string(),
            "01/01/2025U12:00:00".to_string(), "".into());

        let result = db.insert_session_sensor_data(&session_sensor_data);
        assert!(result.is_ok());

        // Check db for new session_sensor
        let fetch_session_sensor_data_result = db.get_session_sensor_data(session_sensor_data.get_id());
        assert!(fetch_session_sensor_data_result.is_ok());

        let fetched_session_sensor_data = fetch_session_sensor_data_result.unwrap();
        assert_models_eq!(session_sensor_data, fetched_session_sensor_data, [get_id, get_datetime, get_blob]);
    }
    */

    #[test]
    fn test_batch_session_sensor_data() {
        let conn = init_schema();

        let db = SqliteDatabase::from_connection(conn).expect("Failed to create test db");
        todo!()
    }

    #[test]
    fn test_get_sessions_sensors_data() {
        let conn = init_schema();

        let db = SqliteDatabase::from_connection(conn).expect("Failed to create test db");
        todo!()
    }

    #[test]
    fn test_get_sessions_sensor_data() {
        let conn = init_schema();

        let db = SqliteDatabase::from_connection(conn).expect("Failed to create test db");
        todo!()
    }

    #[test]
    fn test_get_session_sensor_data() {
        let conn = init_schema();

        let db = SqliteDatabase::from_connection(conn).expect("Failed to create test db");
        todo!()
    }

    #[test]
    fn test_get_session_sensor_datapoint() {
        let conn = init_schema();

        let db = SqliteDatabase::from_connection(conn).expect("Failed to create test db");
        todo!()
    }

    #[test]
    fn test_update_session_sensor_datapoint() {
        let conn = init_schema();

        let db = SqliteDatabase::from_connection(conn).expect("Failed to create test db");
        todo!()
    }

    #[test]
    fn test_delete_session_sensor_datapoint() {
        let conn = init_schema();

        let db = SqliteDatabase::from_connection(conn).expect("Failed to create test db");
        todo!()
    }
}
