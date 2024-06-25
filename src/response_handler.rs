use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};
use std::string::ToString;
use std::time::SystemTime;

use chrono::Utc;

use crate::eprintln_red;
use crate::request_handler::RequestHeader;

//struct ResponseHeader {
//    status: String,
//    server: String,
//    date: String,
//    content_type: String,
//    content_length: String,
//    last_modified: String,
//    etag: String,
//    connection: String,
//    keep_alive: String,
//    accept_ranges: String,
//    transfer_encoding: String,
//    content_encoding: String,
//    vary: String,
//}

pub(crate) struct ResponseHeader {
    status: String,
    server: String,
    content_type: String,
    content_length: String,
    date: String,
}

const IMAGE_FILES: [&str; 2] = ["png", "avif"];
//const BLOCKED_FILES: [&str; 2] = ["php", "ini"];

pub(crate) fn response_builder(req_header: RequestHeader) -> Vec<u8> {
    let (body, status, path) = get_body(req_header);
    //println_cyan!("Body in: {}μs", (Instant::now() - start_time).as_micros());

    let response = ResponseHeader {
        status: format!("HTTP/1.1 {}", &status),
        server: "Server: Polar Bear".to_string(),
        content_type: set_http_content_type(status, &path),
        content_length: format!("Content-Length: {}", body.len()),
        date: set_http_date(),
    };

    response.into_http_response(body)
}


fn get_body(req_header: RequestHeader) -> (Vec<u8>, u16, PathBuf) {
    let path_string = format!(".{}", req_header.http_path);
    let path = Path::new(&path_string).to_owned();

    if path.is_file() {
        match fs::read(&path) {
            Ok(file) => (file, 200, path),
            Err(err) => {
                eprintln_red!("Error while trying to request {} || Msg: {}", req_header.http_path, err);
                ("ERROR: 500 - Internal Server Error!".to_string().into_bytes(), 500, path)
            }
        }
    } else if path.is_dir() {
        let new_path = path.join("index.html");

        if new_path.is_file() {
            match fs::read(&new_path) {
                Ok(file) => (file, 200, new_path),
                Err(err) => {
                    eprintln_red!("Error while trying to request {}{} || Msg: {}", req_header.http_path,"index.html", err);
                    ("ERROR: 500 - Internal Server Error!".to_string().into_bytes(), 500, new_path)
                }
            }
        } else {
            ("ERROR: 404 - Not Found!".to_string().into_bytes(), 404, new_path)
        }
    } else {
        ("ERROR: 404 - Not Found!".to_string().into_bytes(), 404, path)
    }
}


fn set_http_content_type(status: u16, path: &Path) -> String {
    let mut charset = "utf-8".to_string();

    let extension = if status == 200 {
        path.extension().and_then(OsStr::to_str).unwrap_or_default().to_string()
    } else {
        "html".to_string()
    };

    if IMAGE_FILES.contains(&&*extension) {
        charset = String::new();
    }

    let mime_type = match extension.as_str() {
        "html" => "text/html",
        "css" => "text/css",
        "js" => "application/javascript",
        "png" => "image/png",
        _ => "text/plain"
    };

    if !charset.is_empty() {
        charset = format!("; charset={}", charset);
    }

    format!("Content-Type: {mime_type}{charset}")
}

fn set_http_date() -> String {
    let datetime = chrono::DateTime::<Utc>::from(SystemTime::now());
    let formatted_date = datetime.format("%a, %d %b %Y %H:%M:%S GMT");

    format!("date: {}\r\n", formatted_date)
}

impl ResponseHeader {
    fn into_http_response(self, mut body: Vec<u8>) -> Vec<u8> {
        let mut http_header = format!("{}\r\n{}\r\n{}\r\n{}\r\n{}\r\n",
            self.status,
            self.server,
            self.content_type,
            self.content_length,
            self.date
        ).into_bytes();

        http_header.append(&mut body);
        http_header
    }
}