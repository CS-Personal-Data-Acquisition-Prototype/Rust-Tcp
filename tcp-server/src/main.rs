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
//! - [ ] Database interface
//!
mod data;
mod http;
mod models;

use std::fs;
use std::io::Read;
use std::net::{TcpListener, TcpStream};
use std::str;

use data::{Database, MockDatabase, SqliteDatabase};
use http::{HttpHeader, HttpMethod, HttpPath, HttpRequest, HttpResponse, HttpStatus};
use models::User;
use serde_json::json;

const DATABASE_URL: &str = "";

//Result generalization, could replace String with custom error enum
type Result<T> = core::result::Result<T, String>;

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
    let listener = match init_server(Address::IPv4(String::from("127.0.0.1:7878"))) {
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
    /*let database = match SqliteDatabase::new(DATABASE_URL) {
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
            Ok(stream) => {
                handle_connection(database, stream);
            }
            Err(error) => {
                eprintln!("Error occured when establishing connection. Error: {error}");
            }
        });
}

fn handle_connection(database: &dyn Database, mut stream: TcpStream) {
    //allocate a buffer
    let mut buffer = [0; 1024]; //TODO: change size later

    //read the request from the stream to the buffer //TODO: add stream identifier for message
    if let Err(error) = stream.read(&mut buffer) {
        eprintln!("Failed to read from the stream. Error: {error}");
    }

    #[cfg(debug_assertions)]
    //println!("Request size: {}\nRequest: {}", buffer.len(), String::from_utf8_lossy(&buffer[..]));

    //construct a request struct
    let request = HttpRequest::from_request_bytes(&buffer);

    //TODO: validate request credentials from header cookie and get user

    //construct response from possible pathways
    let gen_view = |filename: &str| generate_html_response(String::from("src/views/") + filename);
    let response = match request.path.clone() {
        HttpPath::Index(subpath) => gen_view("index.html"),
        HttpPath::NotFound(path) => HttpResponse::json_404(&path),
        HttpPath::Authentication(subpath) => todo!(),
        HttpPath::User(subpath) => match request.method {
            HttpMethod::Get => match subpath.as_str() {
                "" => {
                    if !database.is_admin() {
                        HttpResponse::not_authorized()
                    } else {
                        match database.get_users() {
                            Ok(users) => HttpResponse::new(
                                HttpStatus::OK,
                                HttpHeader::default_json(),
                                json!({"users": users.iter().map(|user| user.get_username()).collect::<Vec<_>>()}).to_string(),
                            ),
                            Err(_) => HttpResponse::bad_request("Failed to fetch users from database.")
                        }
                    }
                }
                "/profile" => match database
                    .get_session_user(request.headers.get(String::from("session_id")))//TODO: move nested .get to a match
                {
                    //TODO: replace &str with constant
                    Ok(username) => match database.get_user(username) {
                        Ok(user) => HttpResponse::new(
                            HttpStatus::OK,
                            HttpHeader::default_json(),
                            user.public_json()
                        ),
                        Err(_) => HttpResponse::json_404("User"),
                    },
                    Err(_) => HttpResponse::json_404("User"),
                },
                s => {
                    let username = match s[1..].split_once('/') {
                        Some((first, _)) => first.to_string(),
                        None => s[1..].to_string(),
                    };
                    match database.get_user(username) {
                        Ok(user) => todo!(),
                        Err(_) => HttpResponse::json_404(&request.path.to_string()),
                    }
                }
            },
            HttpMethod::Post => match request.body {
                Some(json) => match serde_json::from_value::<User>(json) {
                    Ok(user) => match user.is_valid() {
                        true => match database.insert_user(&user) {
                            Ok(_) => HttpResponse::new(
                                HttpStatus::Created,
                                HttpHeader::default_json(),
                                user.public_json(),
                            ),
                            Err(_) => HttpResponse::bad_request("Error creating user."),
                        },
                        false => HttpResponse::bad_request("Invalid user data."),
                    },
                    Err(_) => HttpResponse::invalid_body(),
                },
                None => HttpResponse::missing_body(),
            },
            HttpMethod::Patch => todo!(),
            HttpMethod::Delete => todo!(),
            HttpMethod::Error => todo!(),
        },
        HttpPath::Sensor(subpath) => todo!(),
        HttpPath::Session(subpath) => todo!(),
        HttpPath::SessionSensor(subpath) => todo!(),
        HttpPath::SessionSensorData(subpath) => todo!(),
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
