use crate::content_fetcher::{file_retriever, path_retriever};
use crate::http::http_init::RequestLine;
use crate::needed::Polar;

#[allow(non_camel_case_types)]
pub(crate) struct http_0_9_response {
    pub(crate) body: Vec<u8>,
}

#[allow(non_camel_case_types)]
pub(crate) struct http_0_9_request {
    pub(crate) request_line: RequestLine,
}

// Welp i only really need to generate a response since http 0.9 doesn't even have http headers lmaooo

pub(crate) fn give_response(request: http_0_9_request) -> Polar<http_0_9_response> {
    let path = path_retriever(request.request_line.uri);

    let file = match path {
        Polar::Some(path_buf) => {
            match file_retriever(path_buf) {
                Polar::Some(file) => {
                    file
                }
                Polar::Silly(code) => return Polar::Silly(code),
            }
        }
        Polar::Silly(code) => return Polar::Silly(code),
    };

    Polar::Some(http_0_9_response {
        body: file,
    })
}

