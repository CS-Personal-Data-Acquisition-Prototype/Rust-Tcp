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
mod http;
mod models;

use std::fs;
use std::io::Read;
use std::net::{TcpListener, TcpStream};
use std::str;

use http::{HttpMethod, HttpRequest, HttpResponse, HttpStatus};

const AUTHENTICATION_ENDPOINT: &str = "/authentication";
const USER_ENDPOINT: &str = "/users";
const SENSOR_ENDPOINT: &str = "/sensors";
const SESSION_ENDPOINT: &str = "/sessions";
const SESSION_SENSOR_ENDPOINT: &str = "/sessions-sensors";
const SESSION_SENSOR_DATA_ENDPOINT: &str = "/sessions-sensors-data";

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

    wait_for_connections(listener);
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
fn wait_for_connections(listener: TcpListener) {
    listener
        .incoming()
        .for_each(|stream_result| match stream_result {
            Ok(stream) => {
                handle_connection(stream);
            }
            Err(error) => {
                eprintln!("Error occured when establishing connection. Error: {error}");
            }
        });
}

fn handle_connection(mut stream: TcpStream) {
    //allocate a buffer
    let mut buffer = [0; 1024]; //TODO: change size later

    //read the request from the stream to the buffer //TODO: add stream identifier for message
    if let Err(error) = stream.read(&mut buffer) {
        eprintln!("Failed to read from the stream. Error: {error}");
    }
    println!("Request size: {}", buffer.len());
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    //construct a request struct
    let request = HttpRequest::from_request_bytes(&buffer);

    //TODO: validate request credentials

    //construct response from possible pathways
    let gen_view = |filename: &str| generate_html_response(String::from("src/views/") + filename);
    let response = match request {
        HttpRequest {
            method: HttpMethod::Get,
            path: p,
            parameters: _,
            body: _,
        } => match p.as_str() {
            "/" => gen_view("index.html"),
            "/page1" => gen_view("index.html"),
            "/page2" => gen_view("index.html"),
            _ => gen_view("404.html"),
        },

        HttpRequest {
            method: HttpMethod::Post,
            path: p,
            parameters: _,
            body: b,
        } => match p.as_str() {
            USER_ENDPOINT => {
                match b {
                    Some(json) => {
                        HttpResponse::new(HttpStatus::Created, json) //TODO: do more with json than echo back
                    }
                    None => HttpResponse::json_404("todo"),
                }
            }
            _ => HttpResponse::json_404("todo"),
        },

        /*HttpRequest {
            method: HttpMethod::Patch,
            path: p,
            body: b,
        } => match p.as_str() {
            _ => gen_res("404.html"),
        },*/
        /*HttpRequest {
            method: HttpMethod::Delete,
            path: p,
            body: _,
        } => match p.as_str() {
            _ => gen_res("404.html"),
        },*/
        _ => HttpResponse::json_404("Resource"),
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

    HttpResponse { status, body }
}
