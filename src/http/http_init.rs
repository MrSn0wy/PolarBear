use crate::needed::Polar;

// references: https://datatracker.ietf.org/doc/html/rfc1945#section-5.1
pub(crate) struct RequestLine {
    pub(crate) method: RequestMethod,
    pub(crate) uri: String,
    pub(crate) version: HttpVersion,
}

pub(crate) struct StatusLine {
    pub(crate) version: HttpVersion,
    pub(crate) status_code: u16,
    pub(crate) reason_phrase: String,
}

#[allow(clippy::upper_case_acronyms)] // why clippy, why
#[derive(Debug, PartialEq)]
pub(crate) enum RequestMethod {
    GET,
    HEAD,
    POST,
    NotImplemented,
}

#[allow(non_camel_case_types)] // I am NOT writing the http versions as "Http11" LMAO
#[derive(Debug)]
pub(crate) enum HttpVersion {
    HTTP_0_9,
    HTTP_1_0,
    HTTP_1_1,
    HTTP_2_0,
    HTTP_3_0,
}

// Goal is to parse the first line, get the method, uri and version, and then send it off to the correct parser, simple right?
pub(crate) fn parser(http_request_line: &String) -> Polar<RequestLine> {
    // step one, get the values
    let temp_vec: Vec<&str> = http_request_line.split(" ").collect();

    let method = match temp_vec.first() {
        Some(method) => *method,
        None => return Polar::Silly(400), // return 400 (bad request)
    };

    let uri = match temp_vec.get(1) {
        Some(uri) => uri.to_string(),
        None => return Polar::Silly(400), // return 400 (bad request)
    };

    let version = match temp_vec.get(2) {
        Some(version) => *version,
        None => "", // do this so if it's http 0.9 it can be handled
    };

    // step two, set the enum's
    let (request_method, http_version) = {
        let request_method = match method {
            "GET" => RequestMethod::GET,
            "HEAD" => RequestMethod::HEAD,
            "POST" => RequestMethod::POST,
            _ => return Polar::Silly(501), // return 501 (not implemented)
        };

        let http_version = match version {
            "HTTP/0.9\r\n" => HttpVersion::HTTP_0_9,
            "HTTP/1.0\r\n" => HttpVersion::HTTP_1_0,
            "HTTP/1.1\r\n" => HttpVersion::HTTP_1_1,
            "HTTP/2.0\r\n" => HttpVersion::HTTP_2_0,
            "HTTP/3.0\r\n" => HttpVersion::HTTP_3_0,
            _ => {
                // check if it is http 0.9
                if version.is_empty() && request_method == RequestMethod::GET {
                    HttpVersion::HTTP_0_9
                } else {
                    return Polar::Silly(505); // return 505 (http version not supported)
                }
            }
        };

        (request_method, http_version)
    };

    // step three, return the struct :D!
    Polar::Some(RequestLine {
        method: request_method,
        uri,
        version: http_version,
    })
}

