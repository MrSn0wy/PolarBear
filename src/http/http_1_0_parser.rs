use std::fs;

use chrono::prelude::*;

use crate::content_fetcher::{file_retriever, path_retriever};

use crate::http::http_init::{HttpVersion, RequestLine, RequestMethod, StatusLine};
use crate::needed::Polar;

// reference: https://datatracker.ietf.org/doc/html/rfc1945#section-6
#[allow(non_camel_case_types)]
pub(crate) struct http_1_0_response {
    pub(crate) status_line: StatusLine,
    pub(crate) general_headers: Vec<GeneralHeader>,
    pub(crate) response_headers: Vec<ResponseHeader>,
    pub(crate) entity_headers: Vec<EntityHeader>,
    pub(crate) body: Vec<u8>,
}


// reference: https://datatracker.ietf.org/doc/html/rfc1945#section-5
#[allow(non_camel_case_types)]
struct http_1_0_request {
    request_line: RequestLine,
    general_headers: Vec<GeneralHeader>,
    request_headers: Vec<RequestHeader>,
    entity_headers: Vec<EntityHeader>,
    body: Option<Vec<u8>>,
}

pub(crate) enum GeneralHeader {
    Date(String),       // http-date of when the message was sent
    Pragma(String),     // this can specify that it shouldn't get cached material
}

enum RequestHeader {
    Authorization(String),      // authorization for the url
    From(String),               // from what email the request was sent from... why is this a thing?
    IfModifiedSince(String),    // http-date
    Referer(String),            // the uri from where the client was before
    UserAgent(String),          // the user agent of the browser, silly
}

pub(crate) enum ResponseHeader {
    Location(String),           // the new location if a file is moved somewhere
    Server(String),             // the name of the server and any info
    WWWAuthenticate(String),    // tells the client how it should authenticate (for 401)
}


// only exists when there is a body :3
pub(crate) enum EntityHeader {
    Allow(Vec<RequestMethod>),          // what request methods is supported on the page, should be ignored with POST
    ContentEncoding(String),            // encodings (e.g. gzip) that the client supports
    ContentLength(u128),                // length of "content" (body) in bytes
    ContentType(String),                // the media-type of the content
    Expires(String),                    // for how long it should be cached, in http-date
    LastModified(String),               // when the client things it was last modified, in http-date
    ExtensionHeader(String, String),    // custom headers that aren't in the http 1.0 spec
}


fn parser(request: Vec<String>, request_line: RequestLine) -> http_1_0_request {
    let mut general_headers = Vec::new();
    let mut request_headers = Vec::new();
    let mut entity_headers = Vec::new();

    for line in request {
        if let Some((header_name, header_value)) = line.split_once(": ") {
            let header_value = header_value.to_string(); // overshadow for ease of use lmao

            match header_name {
                // General Headers
                "Date" => general_headers.push(GeneralHeader::Date(header_value)),
                "Pragma" => general_headers.push(GeneralHeader::Pragma(header_value)),

                // Request Headers
                "Authorization" => request_headers.push(RequestHeader::Authorization(header_value)),
                "From" => request_headers.push(RequestHeader::From(header_value)),
                "If-Modified-Since" => request_headers.push(RequestHeader::IfModifiedSince(header_value)),
                "Referer" => request_headers.push(RequestHeader::Referer(header_value)),
                "User-Agent" => request_headers.push(RequestHeader::UserAgent(header_value)),

                // Entity Headers
                "Allow" => {
                    let methods: Vec<RequestMethod> = header_value
                        .split(", ")
                        .map(|value| match value {
                            "GET" => RequestMethod::GET,
                            "HEAD" => RequestMethod::HEAD,
                            "POST" => RequestMethod::POST,
                            _ => RequestMethod::NotImplemented,
                        }
                        ).collect();

                    entity_headers.push(EntityHeader::Allow(methods))
                }

                "Content-Encoding" => entity_headers.push(EntityHeader::ContentEncoding(header_value)),
                "Content-Length" =>
                    if let Ok(length) = header_value.parse::<u128>() {
                        entity_headers.push(EntityHeader::ContentLength(length));
                    }
                
                "Content-Type" => entity_headers.push(EntityHeader::ContentType(header_value)),
                "Expires" => entity_headers.push(EntityHeader::Expires(header_value)),
                "Last-Modified" => entity_headers.push(EntityHeader::LastModified(header_value)),

                // all the non-spec headers will be seen as entity headers :3
                _ => entity_headers.push(EntityHeader::ExtensionHeader(header_name.to_string(), header_value)), // any unknown headers :D!
            }
        }
    }

    if request_line.method == RequestMethod::POST {
        todo!() // hehe~
    }

    http_1_0_request {
        request_line,
        general_headers,
        request_headers,
        entity_headers,
        body: None,
    }
}


pub(crate) fn return_response(request: Vec<String>, request_line: RequestLine) -> Polar<http_1_0_response> {
    let request = parser(request, request_line);

    let path_buf = match path_retriever(request.request_line.uri) {
        Polar::Some(path_buf) => path_buf,
        Polar::Silly(code) => return Polar::Silly(code),
    };

    let metadata = match fs::metadata(&path_buf) {
        Ok(data) => data,
        Err(_) => return Polar::Silly(500),
    };

    let file = match file_retriever(&path_buf) {
        Polar::Some(file) => file,
        Polar::Silly(code) => return Polar::Silly(code),
    };


    let general_headers: Vec<GeneralHeader> = vec![
        GeneralHeader::Date({
            let now: DateTime<Utc> = Utc::now();    // Get the current UTC time
            now.format("%a, %d %b %Y %H:%M:%S GMT").to_string() // Format da time as HTTP-date
        }),
        GeneralHeader::Pragma("no-cache".to_string()),
    ];

    let response_headers: Vec<ResponseHeader> = vec![
        //ResponseHeader::Location(),
        ResponseHeader::Server("PolarBear".to_string()),
        //ResponseHeader::WWWAuthenticate(),
    ];


    let entity_headers: Vec<EntityHeader> = vec![
        EntityHeader::Allow(vec![RequestMethod::GET]),
        //EntityHeader::ContentEncoding()
        EntityHeader::ContentLength(file.len() as u128),
        EntityHeader::ContentType(
            //todo! change this to metadata? maybe?
            match path_buf.extension().and_then(|ext| ext.to_str()) {
                None => { "text/plain; charset=utf-8".to_string() }
                Some(extension) => match extension {
                    "html" => "text/html; charset=utf-8".to_string(),
                    "css" => "text/css; charset=utf-8".to_string(),
                    "js" => "text/javascript; charset=utf-8".to_string(),
                    "txt" => "text/plain; charset=utf-8".to_string(),

                    "png" => "image/png".to_string(),
                    "apng" => "image/apng".to_string(),
                    "avif" => "image/avif".to_string(),
                    "gif" => "image/gif".to_string(),
                    "jpeg" => "image/jpeg".to_string(),
                    "jpg" => "image/jpeg".to_string(),
                    "svg" => "image/svg+xml".to_string(),
                    "webp" => "image/webp".to_string(),

                    "mp4" => "video/mp4".to_string(),
                    "av1" => "video/AV1".to_string(),

                    _ => "text/plain; charset=utf-8".to_string(),
                }
            }),
        //EntityHeader::Expires()
        EntityHeader::LastModified(metadata
            .modified()
            .map(|time| {
                let datetime: DateTime<Utc> = time.into(); // Convert `SystemTime` to the `DateTime<Utc>`
                datetime.format("%a, %d %b %Y %H:%M:%S GMT").to_string() // Format the time in HTTP-date format
            })
            .unwrap_or({ // Fallback to a default date
                let now: DateTime<Utc> = Utc::now();    // Get the current UTC time
                now.format("%a, %d %b %Y %H:%M:%S GMT").to_string() // Format da time as HTTP-date
            })
        ),
    ];


    Polar::Some(http_1_0_response {
        status_line: StatusLine {
            version: HttpVersion::HTTP_1_0,
            status_code: 200,
            reason_phrase: "OK".to_string(),
        },
        general_headers,
        response_headers,
        entity_headers,
        body: file,
    })
}


impl http_1_0_response {
    pub(crate) fn into_response(mut self) -> Vec<u8> {
        let mut response: Vec<u8> = vec![];
        // status line
        response.append(&mut format!("HTTP/1.0 {} {} \r\n", self.status_line.status_code, self.status_line.reason_phrase).into_bytes());

        // da headers
        for general_header in self.general_headers {
            match general_header {
                GeneralHeader::Date(data) => response.append(&mut format!("Date: {}\r\n", data).into_bytes()),
                GeneralHeader::Pragma(data) => response.append(&mut format!("Pragma: {}\r\n", data).into_bytes()),
            }
        }

        for response_header in self.response_headers {
            match response_header {
                ResponseHeader::Location(data) => response.append(&mut format!("Location: {}\r\n", data).into_bytes()),
                ResponseHeader::Server(data) => response.append(&mut format!("Server: {}\r\n", data).into_bytes()),
                ResponseHeader::WWWAuthenticate(data) => response.append(&mut format!("WWW-Authenticate: {}\r\n", data).into_bytes()),
            }
        }

        for entity_header in self.entity_headers {
            match entity_header {
                EntityHeader::Allow(data) => {
                    let data: String = data
                        .iter()
                        .map(|value| match value {
                            RequestMethod::GET => "GET".to_string(),
                            RequestMethod::HEAD => "HEAD".to_string(),
                            RequestMethod::POST => "POST".to_string(),
                            RequestMethod::NotImplemented => "".to_string(),
                        })
                        .collect::<Vec<String>>()
                        .join(", ");

                    response.append(&mut format!("Allow: {}\r\n", data).into_bytes())
                }
                EntityHeader::ContentEncoding(data) => response.append(&mut format!("Content-Encoding: {}\r\n", data).into_bytes()), //todo: make modular since multiple can be defined
                EntityHeader::ContentLength(data) => response.append(&mut format!("Content-Length: {}\r\n", data).into_bytes()),
                EntityHeader::ContentType(data) => response.append(&mut format!("Content-Type: {}\r\n", data).into_bytes()),
                EntityHeader::Expires(data) => response.append(&mut format!("Expires: {}\r\n", data).into_bytes()),
                EntityHeader::LastModified(data) => response.append(&mut format!("Last-Modified: {}\r\n", data).into_bytes()),
                EntityHeader::ExtensionHeader(_, _) => {} // todo: will not be processed, should prob change this but whatever
            }
        }
        response.append(&mut "\r\n".to_string().into_bytes());
        // body
        response.append(&mut self.body);

        // return response :3
        response
    }
}



