use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;

use crate::println_green;
use crate::response_handler::response_builder;

//use native_tls::TlsStream;

pub(crate) enum RequestType {
    Https(TcpStream),
    Http(TcpStream),
}

pub(crate) struct RequestHeader {
    pub(crate) http_type: String,
    pub(crate) http_path: String,
    pub(crate) http_version: String,
    pub(crate) accepts: Vec<String>,
    //host: String,
    //user_agent: String,
    //accept: String,
    //accept_language: String,
    //accept_encoding: String,
    //connection: String,
    //upgrade_insecure_requests: String,
}

pub(crate) fn request_handler(request_type: RequestType) -> anyhow::Result<()> {
    match request_type {
        RequestType::Https(mut stream) => {
            let buf_reader = BufReader::new(&mut stream);

            let http_request: Vec<String> = buf_reader
                .lines()
                .map(|result| result.unwrap_or_default())
                .take_while(|line| !line.is_empty())
                .collect();

            let req_header: RequestHeader = match RequestHeader::parse_http_request(&http_request) {
                Ok(req) => req,
                Err(_) => todo!()
            };

            println_green!("Request received: type={} path={} version={}", &req_header.http_type, &req_header.http_path, &req_header.http_version);

            let response = response_builder(req_header);
            stream.write_all(&response)?;

            stream.flush()?;
        }
        RequestType::Http(mut stream) => {
            let buf_reader = BufReader::new(&mut stream);

            let http_request: Vec<String> = buf_reader
                .lines()
                .map(|result| result.unwrap_or_default())
                .take_while(|line| !line.is_empty())
                .collect();

            let req_header: RequestHeader = match RequestHeader::parse_http_request(&http_request) {
                Ok(req) => req,
                Err(_) => todo!()
            };

            println_green!("Request received: type={} path={} version={}", &req_header.http_type, &req_header.http_path, &req_header.http_version);

            let response = response_builder(req_header);
            stream.write_all(&response)?;

            stream.flush()?;
        }
    }

    Ok(())
}


impl RequestHeader {
    pub(crate) fn parse_http_request(request_header: &Vec<String>) -> anyhow::Result<RequestHeader> {
        let mut request = Self {
            http_type: "".to_string(),
            http_path: "".to_string(),
            http_version: "".to_string(),
            accepts: Vec::new(),
        };

        for line in request_header {
            if line.starts_with("GET") {
                let splitter_line = line.split(' ').collect::<Vec<&str>>();

                request.http_type = splitter_line[0].to_string();
                request.http_path = splitter_line[1].to_string();
                request.http_version = splitter_line[2].to_string();
            } else if line.starts_with("accept:") {
                let splitter_line = line.split(',').map(|x| x.to_string()).collect::<Vec<String>>();
                request.accepts = splitter_line
            }
        }

        Ok(request)
    }
}