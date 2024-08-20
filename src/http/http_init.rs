use crate::needed::Polar;

// references: https://datatracker.ietf.org/doc/html/rfc1945#section-5.1
pub(crate) struct RequestLine {
    pub(crate) method: RequestMethod,
    pub(crate) uri: String,
    pub(crate) version: HttpVersion,
}

#[allow(clippy::upper_case_acronyms)] // why clippy, why
#[derive(Debug)]
pub(crate) enum RequestMethod {
    GET,
    HEAD,
    POST,
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

    let uri = temp_vec.get(1).unwrap_or(&"/").to_string(); // when it is empty, the server should treat is as "/"
    let version = *temp_vec.get(2).unwrap_or(&""); // todo! trim leading 0 from the version? idk, rfc is silly


    // step two, set the enum's
    let (request_method, http_version) =
        // HTTP 0.9 doesn't have a version indicator and only supports GET requests.
        if version.is_empty() && method == "GET" {
            let request_method = RequestMethod::GET;
            let http_version = HttpVersion::HTTP_0_9;

            (request_method, http_version)
        } else {
            // now we can do the normal http checking for other version, currently only checks for http 1.0 request methods!
            let request_method = match method {
                "GET" => RequestMethod::GET,
                "HEAD" => RequestMethod::HEAD,
                "POST" => RequestMethod::POST,
                _ => return Polar::Silly(501), // return 501 (not implemented)
            };

            let http_version = match version {
                "HTTP/1.0\r\n" => HttpVersion::HTTP_1_0,
                "HTTP/1.1\r\n" => HttpVersion::HTTP_1_1,
                "HTTP/2.0\r\n" => HttpVersion::HTTP_2_0,
                "HTTP/3.0\r\n" => HttpVersion::HTTP_3_0,
                _ => return Polar::Silly(505), // return 505 (http version not supported)
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

