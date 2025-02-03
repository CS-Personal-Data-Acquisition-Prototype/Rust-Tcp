use std::{io::Write, net::TcpStream};

use super::{HttpHeader, HttpStatus};

pub struct HttpResponse {
    pub status: HttpStatus,
    pub headers: HttpHeader,
    pub body: String,
}

impl HttpResponse {
    pub fn html_404() -> HttpResponse {
        HttpResponse {
            status: HttpStatus::NotFound,
            headers: HttpHeader::default_html(),
            body: String::from("<html><body><h1>404 Not Found</h1></body></html>"),
        }
    }

    pub fn json_404(resource: &str) -> HttpResponse {
        HttpResponse {
            status: HttpStatus::NotFound,
            headers: HttpHeader::default_json(),
            body: format!("{{ \"error\":\"{resource} not found\" }}"),
        }
    }

    pub fn new(status: HttpStatus, header: HttpHeader, body: String) -> HttpResponse {
        HttpResponse {
            status,
            headers: header,
            body: body.trim_end_matches('\0').to_string(),
        }
    }

    pub fn to_string(&self) -> String {
        format!(
            "HTTP/1.1 {}\r\nContent-Length: {}\r\n{}\r\n\r\n{}",
            self.status.to_string(),
            self.body.len(),
            self.headers.to_string(),
            self.body,
        )
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        self.to_string().into_bytes()
    }

    pub fn send(&self, mut stream: TcpStream) -> Result<(), String> {
        let data: &[u8] = &self.to_bytes();
        let len: usize = data.len();
        let mut remaining_bytes = len;
        let mut head = 0;

        while remaining_bytes > 0 {
            match stream.write(&data[head..]) {
                Ok(0) => break,
                Ok(n) => {
                    remaining_bytes -= n;
                    head += n;
                    if let Err(error) = stream.flush() {
                        return Err(format!(
                            "Failed to send data, sent {}/{} bytes. Error: {}",
                            len - remaining_bytes,
                            len,
                            error
                        ));
                    }
                }
                Err(error) => {
                    return Err(format!(
                        "Failed to send data, sent {}/{} bytes. Error: {}",
                        len - remaining_bytes,
                        len,
                        error
                    ));
                }
            }
        }
        Ok(())
    }
}
