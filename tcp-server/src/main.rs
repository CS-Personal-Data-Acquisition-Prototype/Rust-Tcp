//! Single threaded TCP Server.
//!
//! ## Intoduction
//!
//! Make intoduction as crate gets more complicated.
//!
//! ## Features
//!
//! - [x] Single thread server
//! - [x] HttpRequest struct deserialization from raw requests
//! - [x] Response generation from files
//! - [x] Route handling with respect to method, path, and body
//! - [ ] Request query string parsed
//! - [ ] Multithread with pooling
//! - [x] Database interface
//!
mod data;
mod http;
mod models;

use std::fs;
use std::io::Read;
use std::net::{TcpListener, TcpStream};
use std::str;

use data::{Database, MockDatabase};
use http::http_header::HttpHeaderType;
use http::{HttpHeader, HttpMethod, HttpPath, HttpRequest, HttpResponse, HttpStatus};
use models::{BaseModel, Sensor, Session, SessionSensor, SessionSensorData, User};
use serde::Deserialize;
use serde_json::json;

//Result generalization, could replace String with custom error enum
type Result<T> = core::result::Result<T, String>;

const HTTP_HEADER_DELIMITER: &[u8] = b"\r\n\r\n";

#[allow(unused)]
#[derive(Deserialize)]
struct Config {
    database_path: String,
    local_addr: String,
}

#[allow(unused)]
enum Address {
    IPv4(String),
    IPv6(String),
}

impl Address {
    fn to_string(&self) -> &String {
        match self {
            Address::IPv4(addr) | Address::IPv6(addr) => addr,
        }
    }
}

fn main() {
    let config = match std::env::current_dir() {
        Ok(mut path) => {
            path.push("src");
            path.push("config.toml");
            match fs::read_to_string(&path) {
                Ok(s) => match toml::from_str::<Config>(&s) {
                    Ok(c) => c,
                    Err(e) => panic!(
                        "Failed to parse config.toml at {:?} into valid toml: {e}",
                        path
                    ),
                },
                Err(e) => panic!("Failed to read config.toml file at {:?}: {e}", path),
            }
        }
        Err(e) => panic!("Failed to get current directory: {e}"),
    };
    let listener = match init_server(Address::IPv4(config.local_addr)) {
        Ok((tcp_listener, address)) => {
            println!("Server listening on '{}'", address.to_string());
            tcp_listener
        }
        Err(error) => {
            eprintln!("{error}");
            return;
        }
    };

    let database = MockDatabase::new();
    /*let database = match data::SqliteDatabase::new(&config.database_path) {
        Ok(db) => db,
        Err(error) => {
            println!("Failed to establish database connection. Error: {error}");
            return;
        },
    };*/

    wait_for_connections(&database, listener);
}

//Returns a tcp listener on success or error string on failure
fn init_server(address: Address) -> Result<(TcpListener, Address)> {
    let addr_str = address.to_string();
    Ok((
        TcpListener::bind(addr_str).map_err(|error| {
            format!("Failed to bind server at address {addr_str}, Error: {error}")
        })?,
        address,
    ))
}

//Forever wait for connections on the listener
fn wait_for_connections(database: &dyn Database, listener: TcpListener) {
    listener
        .incoming()
        .for_each(|stream_result| match stream_result {
            Ok(stream) => handle_connection(database, stream),
            Err(error) => eprintln!("Error occured when establishing connection. Error: {error}"),
        });
}

fn handle_connection(database: &dyn Database, mut stream: TcpStream) {
    // allocate buffer to hold request
    let mut buffer = vec![0; 1_024]; //1_500_000
    let mut total_bytes = 0;
    let mut tries = 4;

    // read request to buffer until HTTP header delimiter is read
    println!("Starting to read from stream");
    let request_option: Result<HttpRequest> = loop {
        match stream.read(&mut buffer[total_bytes..]) {
            Ok(0) => {
                tries -= 1;
                if tries <= 0 {
                    break Err(format!("Connection closed after {tries} attempts."));
                }
                std::thread::sleep(std::time::Duration::from_millis(500));
            }
            Ok(n) => {
                tries = 3;
                total_bytes += n;

                // if header delimiter is found
                if let Some(delim_index) = buffer
                    .windows(HTTP_HEADER_DELIMITER.len())
                    .position(|window| window == HTTP_HEADER_DELIMITER)
                    .map(|pos| pos + HTTP_HEADER_DELIMITER.len())
                {
                    //construct a request struct before reading the body
                    let mut parsed_request =
                        HttpRequest::from_request_bytes(&buffer[..delim_index]);

                    //TODO: validate origin
                    /*match parsed_request.headers.get(HttpHeaderType::Origin.as_str()) {
                        Some(origin) if origin == HttpHeader::AC_ORIGIN => {}
                        Some(invalid_origin) => break Err(format!(
                                "Request origin invalid: expected '{}', recieved '{invalid_origin}'",
                                HttpHeader::AC_ORIGIN
                            )),
                        None => break Err(format!(
                            "Request origin invalid: expected '{}', recieved None",
                            HttpHeader::AC_ORIGIN
                        )),
                    }*/

                    let body_size = match parsed_request
                        .headers
                        .get(HttpHeaderType::ContentLength.as_str())
                    {
                        Some(length) => {
                            match length.trim().parse::<usize>() {
                                Ok(num) => num,
                                Err(e) => break Err(format!(
                                    "Failed to parse '{}' header to usize: {e}",
                                    HttpHeaderType::ContentLength.as_str()
                                ))
                            }
                        }
                        None => {
                            println!(
                                "No '{}' header found",
                                HttpHeaderType::ContentLength.as_str()
                            );
                            0
                        }
                    };

                    if body_size > 0 {
                        let body_recieved = total_bytes - delim_index;
                        let request_len = total_bytes - body_recieved + body_size;

                        // expand buffer to fit rest of request
                        buffer.resize(buffer.len().max(request_len), 0u8);

                        // read rest of body not recieved [total_bytes, body_size - body_recieved]
                        if let Err(e) = stream.read_exact(
                            &mut buffer[total_bytes..request_len],
                        ) {
                            eprintln!(
                                "Error encountered when reading '{body_size}' bytes from the stream: {e}"
                            )
                        }
                        total_bytes += body_size - body_recieved;

                        // add body into request struct
                        if let Err(e) = parsed_request
                            .parse_body(&buffer[delim_index..(delim_index + body_size)])
                        {
                            eprintln!("Failed to parse request body: {e}");
                        }
                    }

                    #[cfg(debug_assertions)]
                    let _ = fs::write("last_request.txt", parsed_request.to_string());
                    break Ok(parsed_request);
                }
            }
            Err(e) => eprintln!("Failed to read from the stream: {e}"),
        }
    };
    println!("{total_bytes} total bytes read\n");

    let response = match request_option {
        Err(e) => HttpResponse::bad_request(&format!("Failed to parse request on the server: {e}")),
        Ok(request) => {
            //construct response from possible pathways
            let gen_view =
                |filename: &str| generate_html_response(String::from("src/views/") + filename);

            match request.path.clone() {
                HttpPath::Index(_subpath) => gen_view("index.html"),
                HttpPath::NotFound(path) => HttpResponse::json_404(&path),
                HttpPath::Authentication(subpath) => match request.method {
                    HttpMethod::Post => match subpath.as_str() {
                        "/login" => match request.body {
                            Some(json) => match User::from_json(json) {
                                Ok(user) => match database.login(&user) {
                                    Ok(session_id) => HttpResponse::new(
                                        HttpStatus::NoContent,
                                        HttpHeader::default_json().set_session(session_id),
                                        String::new(),
                                    ),
                                    Err(_) => HttpResponse::not_authorized(),
                                },
                                Err(msg) => HttpResponse::invalid_body(Some(&msg)),
                            },
                            None => HttpResponse::missing_body(Some(User::REQUIRED_VALUES)),
                        },
                        "/logout" => match request.headers.get_cookie(HttpHeaderType::SessionID.as_str()) {
                            Some(session_id) => match database.logout(&session_id) {
                                Ok(_) => HttpResponse::new(
                                    HttpStatus::NoContent,
                                    HttpHeader::default_json().set_session(String::new()),
                                    String::new(),
                                ),
                                Err(_) => HttpResponse::json_404("Session"),
                            },
                            None => HttpResponse::not_authorized(),
                        },
                        "/renew" => match request.headers.get_cookie(HttpHeaderType::SessionID.as_str()) {
                            Some(session_id) => match database.renew_session(&session_id) {
                                Ok(new_session_id) => HttpResponse::new(
                                    HttpStatus::NoContent,
                                    HttpHeader::default_json().set_session(new_session_id),
                                    String::new(),
                                ),
                                Err(_) => HttpResponse::json_404("Session"),
                            },
                            None => HttpResponse::not_authorized(),
                        },
                        _ => HttpResponse::json_404(&request.path.to_string()),
                    },
                    _ => HttpResponse::json_404(&request.path.to_string()),
                },
                HttpPath::User(subpath) => match request.method {
                    HttpMethod::Get => match HttpPath::subsection(&subpath, 0) {
                        None => match request.headers.get_cookie(HttpHeaderType::SessionID.as_str()) {
                            Some(session_id) => match database.get_session_user(&session_id) {
                                Ok(user) => {
                                    if !database.is_admin(&user) {
                                        HttpResponse::forbidden()
                                    } else {
                                        match database.get_users() {
                                        Ok(users) => HttpResponse::from_vec(
                                            json!({"users": users.iter().map(|user| user.get_username()).collect::<Vec<_>>()}).to_string()
                                        ),
                                        Err(_) => HttpResponse::bad_request("Failed to fetch users from database.")
                                    }
                                    }
                                }
                                Err(_) => HttpResponse::not_authorized(),
                            },
                            None => HttpResponse::not_authorized(),
                        },
                        Some("profile") => match request.headers.get_cookie(HttpHeaderType::SessionID.as_str()) {
                            Some(session_id) => match database.get_session_user(&session_id) {
                                Ok(user) => user.to_ok_response(),
                                Err(_) => HttpResponse::json_404("User"),
                            },
                            None => HttpResponse::not_authorized(),
                        },
                        Some(username) => match database.get_user(username) {
                            Ok(user) => user.to_ok_response(),
                            Err(_) => HttpResponse::json_404(&request.path.to_string()),
                        },
                    },
                    HttpMethod::Post => User::insert_model(database, request.body),
                    HttpMethod::Patch => User::update_model(database, &subpath, request.body),
                    HttpMethod::Delete => User::delete_model(database, &subpath),
                    HttpMethod::Options => HttpResponse::options_response(),
                    HttpMethod::Error => HttpResponse::json_404(&request.path.to_string()),
                },
                HttpPath::Sensor(subpath) => {
                    match request.method {
                        HttpMethod::Get => {
                            match HttpPath::subsection(&subpath, 0) {
                                None => match request.headers.get_cookie(HttpHeaderType::SessionID.as_str()) {
                                    Some(session_id) => match database.get_session_user(&session_id) {
                                        Ok(user) => {
                                            if !database.is_admin(&user) {
                                                HttpResponse::forbidden()
                                            } else {
                                                match database.get_sensors() {
                                                    Ok(sensors) => HttpResponse::from_vec(json!({"sensors": sensors.iter().map(|sensor| json!({
                                                        "id": sensor.get_id(),
                                                        "type": sensor.get_sensor_type()
                                                    })).collect::<Vec<_>>()}).to_string()),
                                                    Err(_) => HttpResponse::bad_request("failed to fetch sensors from database."),
                                                }
                                            }
                                        }
                                        Err(_) => HttpResponse::not_authorized(),
                                    },
                                    None => HttpResponse::not_authorized(),
                                },
                                Some(sensor_id) => match database.get_sensor(sensor_id) {
                                    Ok(sensor) => sensor.to_ok_response(),
                                    Err(_) => HttpResponse::json_404(&request.path.to_string()),
                                },
                            }
                        }
                        HttpMethod::Post => Sensor::insert_model(database, request.body),
                        HttpMethod::Patch => Sensor::update_model(database, &subpath, request.body),
                        HttpMethod::Delete => Sensor::delete_model(database, &subpath),
                        HttpMethod::Options => HttpResponse::options_response(),
                        HttpMethod::Error => HttpResponse::json_404(&request.path.to_string()),
                    }
                }
                HttpPath::Session(subpath) => {
                    match request.method {
                        HttpMethod::Get => match HttpPath::subsection(&subpath, 0) {
                            None => match request.headers.get_cookie(HttpHeaderType::SessionID.as_str()) {
                                Some(session_id) => match database.get_session_user(&session_id) {
                                    Ok(user) => {
                                        if !database.is_admin(&user) {
                                            HttpResponse::forbidden()
                                        } else {
                                            match database.get_all_sessions() {
                                        Ok(sessions) => HttpResponse::from_vec(json!({"sessions": sessions.iter().map(|session| json!({
                                            "session_id": session.get_id(),
                                            "username": session.get_username()
                                        })).collect::<Vec<_>>()}).to_string()),
                                        Err(_) => HttpResponse::bad_request("failed to fetch sessions from the database."),
                                    }
                                        }
                                    }
                                    Err(_) => HttpResponse::not_authorized(),
                                },
                                None => HttpResponse::not_authorized(),
                            },
                            Some("user") => match HttpPath::subsection(&subpath, 1) {
                                Some(username) => match database.get_user_sessions(&username) {
                                    Ok(sessions) => HttpResponse::from_vec(
                                        json!({"sessions": sessions.iter().map(|session| json!({
                                        "session_id": session.get_id(),
                                        "username": session.get_username()
                                    })).collect::<Vec<_>>()})
                                        .to_string(),
                                    ),
                                    Err(_) => HttpResponse::bad_request(
                                        "failed to fetch user sessions from the database.",
                                    ),
                                },
                                None => HttpResponse::json_404(&request.path.to_string()),
                            },
                            Some("id") => match HttpPath::subsection(&subpath, 1) {
                                Some(session_id) => match database.get_session(&session_id) {
                                    Ok(session) => session.to_ok_response(),
                                    Err(_) => HttpResponse::bad_request(
                                        "failed to fetch session from the database.",
                                    ),
                                },
                                None => HttpResponse::json_404(&request.path.to_string()),
                            },
                            _ => HttpResponse::json_404(&request.path.to_string()),
                        },
                        HttpMethod::Post => Session::insert_model(database, request.body),
                        HttpMethod::Patch => Session::update_model(database, &subpath, request.body),
                        HttpMethod::Delete => Session::delete_model(database, &subpath),
                        HttpMethod::Options => HttpResponse::options_response(),
                        HttpMethod::Error => HttpResponse::json_404(&request.path.to_string()),
                    }
                }
                HttpPath::SessionSensor(subpath) => match request.method {
                    HttpMethod::Get => match HttpPath::subsection(&subpath, 0) {
                        None => match request.headers.get_cookie(HttpHeaderType::SessionID.as_str()) {
                            Some(session_id) => match database.get_session_user(&session_id) {
                                Ok(user) => {
                                    if !database.is_admin(&user) {
                                        HttpResponse::forbidden()
                                    } else {
                                        match database.get_sessions_sensors() {
                                            Ok(sessions_sensors) => HttpResponse::from_vec(json!({"sessions_sensors": sessions_sensors.iter().map(|session_sensor| {
                                                json!({
                                                    "id": session_sensor.get_id(),
                                                    "session_id": session_sensor.get_session_id(),
                                                    "sensor_id": session_sensor.get_sensor_id(),
                                                })
                                            }).collect::<Vec<_>>()}).to_string()),
                                            Err(_) => HttpResponse::bad_request(
                                                "failed to fetch sessions sensors from the database.",
                                            ),
                                        }
                                    }
                                }
                                Err(_) => HttpResponse::not_authorized(),
                            },
                            None => HttpResponse::not_authorized(),
                        },
                        Some("session") => match HttpPath::subsection(&subpath, 1) {
                            Some(session_id) => match database.get_session_sensors(session_id) {
                                Ok(session_sensors) => HttpResponse::from_vec(json!({"sessions_sensors": session_sensors.iter().map(|session_sensor| {
                                    json!({
                                        "id": session_sensor.get_id(),
                                        "session_id": session_sensor.get_session_id(),
                                        "sensor_id": session_sensor.get_sensor_id(),
                                    })
                                }).collect::<Vec<_>>()}).to_string()),
                                Err(_) => HttpResponse::bad_request(
                                    "failed to fetch session sensors from the database.",
                                ),
                            },
                            None => HttpResponse::json_404(&request.path.to_string()),
                        },
                        Some("session-sensor") => match HttpPath::subsection(&subpath, 1) {
                            Some(session_sensor_id) => match database.get_session_sensor(session_sensor_id) {
                                Ok(session_sensor) => session_sensor.to_ok_response(),
                                Err(_) => HttpResponse::bad_request(
                                    "failed to fetch session sensor from the database.",
                                ),
                            },
                            None => HttpResponse::json_404(&request.path.to_string()),
                        }
                        _ => HttpResponse::json_404(&request.path.to_string()),
                    },
                    HttpMethod::Post => SessionSensor::insert_model(database, request.body),
                    HttpMethod::Patch => SessionSensor::update_model(database, &subpath, request.body),
                    HttpMethod::Delete => SessionSensor::delete_model(database, &subpath),
                    HttpMethod::Options => HttpResponse::options_response(),
                    HttpMethod::Error => HttpResponse::json_404(&request.path.to_string()),
                },
                HttpPath::SessionSensorData(subpath) => match request.method {
                    HttpMethod::Get => match HttpPath::subsection(&subpath, 0) {
                        None => match request.headers.get_cookie(HttpHeaderType::SessionID.as_str()) {
                            Some(session_id) => match database.get_session_user(&session_id) {
                                Ok(user) => {
                                    if !database.is_admin(&user) {
                                        HttpResponse::forbidden()
                                    } else {
                                        match database.get_sessions_sensors_data() {
                                        Ok(sessions_sensors_data) => HttpResponse::from_vec(json!({ "datapoints": sessions_sensors_data.iter().map(|session_sensor_data| {
                                            json!({
                                                "id": session_sensor_data.get_id(),
                                                "datetime": session_sensor_data.get_datetime(),
                                                "data_blob": session_sensor_data.get_blob(),
                                            })
                                        }).collect::<Vec<_>>()}).to_string()),
                                        Err(_) => todo!(),
                                    }
                                    }
                                }
                                Err(_) => HttpResponse::not_authorized(),
                            },
                            None => HttpResponse::not_authorized(),
                        },
                        Some("session") => match HttpPath::subsection(&subpath, 1) {
                            Some(session_id) => match database.get_sessions_sensor_data(session_id) {
                                Ok(sessions_sensor_data) => HttpResponse::from_vec(json!({ "datapoints": sessions_sensor_data.iter().map(|session_sensor_data| {
                                    json!({
                                        "id": session_sensor_data.get_id(),
                                        "datetime": session_sensor_data.get_datetime(),
                                        "data_blob": session_sensor_data.get_blob(),
                                    })
                                }).collect::<Vec<_>>()}).to_string()),
                                Err(_) => HttpResponse::json_404(&request.path.to_string()),
                            },
                            None => HttpResponse::json_404(&request.path.to_string()),
                        },
                        Some("id") => match HttpPath::subsection(&subpath, 1) {
                            Some(session_sensor_id) => match database.get_session_sensor_data(session_sensor_id) {
                                Ok(session_sensor_data) => HttpResponse::from_vec(json!({ "datapoints": session_sensor_data.iter().map(|session_sensor_data| {
                                    json!({
                                        "id": session_sensor_data.get_id(),
                                        "datetime": session_sensor_data.get_datetime(),
                                        "data_blob": session_sensor_data.get_blob(),
                                    })
                                }).collect::<Vec<_>>()}).to_string()),
                                Err(_) => HttpResponse::json_404(&request.path.to_string()),
                            },
                            None => HttpResponse::json_404(&request.path.to_string()),
                        },
                        Some(session_sensor_id) => match HttpPath::subsection(&subpath, 1) {
                            Some(datetime) => match database.get_session_sensor_datapoint(session_sensor_id, datetime) {
                                Ok(datapoint) => datapoint.to_ok_response(),
                                Err(_) => HttpResponse::json_404(&request.path.to_string()),
                            },
                            None => HttpResponse::json_404(&request.path.to_string()),
                        },
                    },
                    HttpMethod::Post => match subpath.as_str() {
                        "" => SessionSensorData::insert_model(database, request.body),
                        "/batch" => SessionSensorData::try_batch_model(database, request.body),
                        _ => HttpResponse::json_404(&request.path.to_string()),
                    },
                    HttpMethod::Patch => SessionSensorData::update_model(database, &subpath, request.body),
                    HttpMethod::Delete => SessionSensorData::delete_model(database, &subpath),
                    HttpMethod::Options => HttpResponse::options_response(),
                    HttpMethod::Error => HttpResponse::json_404(&request.path.to_string()),
                },
            }
        }
    };

    //send generated response //TODO: add stream identifier for error message
    if let Err(error) = response.send(stream) {
        eprintln!("Failed to send response to stream. Error: {error}")
    }
}

fn generate_html_response(path: String) -> HttpResponse {
    //read content file
    let (status, body) = match fs::read_to_string(&path) {
        Ok(content) => (HttpStatus::OK, content),
        Err(error) => {
            eprintln!("Error reading file {path}. Error: {error}");
            return HttpResponse::html_404();
        }
    };

    HttpResponse {
        status,
        headers: HttpHeader::default_html(),
        body,
    }
}
