use std::time::{SystemTime, UNIX_EPOCH};

use chrono::{DateTime, Utc};

use crate::http::http_1_0_parser::{EntityHeader, http_1_0_response};
use crate::http::http_init::{HttpVersion, StatusLine};
use crate::http::http_init::RequestMethod::GET;

// 400 - Bad Request
const FUNNY_400_MESSAGES: &[&str] = &[
    "The request was so funny, the server forgot to laugh.",
    "Idk what you had, but can i have some too?",
    "Maybe take a break, you deserve it :D!",
];

// 404 - Not Found
const FUNNY_404_MESSAGES: &[&str] = &[
    "Idk what you want, but you ain't gettin it!",
    "It appears this page doesnt exist, are you a time traveler?!",
    "I think you took a wrong turn bud!",
    "Please be a bit more clear next time, ey?",
    "Did you mistake an I for an l?",
    "Let me guess, the cat walked over the keyboard?",
    "We might've gotten lost"
];

// 501 - Not Implemented
const FUNNY_501_MESSAGES: &[&str] = &[
    "Either you put in some gibberish, or i haven't implemented it yet... I think its the first one",
    "You should be nicer to the server you know.. It is trying it's best",
    "It appears your browser was yapping too much and the server had no idea what it was saying!",
];

// 505 - HTTP Version Not Supported
const FUNNY_505_MESSAGES: &[&str] = &[
    "To be honest, i have no clue how you caused this, so props to you stranger!",
    "I know what you did",
    "How is HTTP 4.0?",
    "Are you running MSDOS?"
];

const ERROR_HTML: &str =
    "<!DOCTYPE html>
<html lang=\"en\">
<head>
    <meta charset=\"UTF-8\">
    <title>Polar Bear</title>
    <link rel=\"icon\" href=\"pfp.png\" type=\"image/png\">

    <style>
        body {
            background-color: #363b33;
            margin-top: 8%;
            font-size: 24px;
            color: #eceff4;
            font-family: Arial, serif;
            font-weight: 700;
            text-align: center;
            display: flex;
            flex-direction: column;
        }
    </style>
</head>
<body>
    <span style=\"font-size: 20px;\"><span style=\"color: #b3d1e7\">Polar Bear</span> had an error while processing that request! <i style=\"font-size: 12px\">sorry!</i></span>
    <span style=\"margin-top: 8vh; color: #b3d1e7\">{set error code here owo}</span>
    <span style=\"font-size: 16px; margin-top: 1.5vh; color: #8f8f8f\">{set random error message here owo}</span>
</body>
</html>";

pub(crate) fn handle_error_codes(http_code: u16) -> Vec<u8> {
    let (body, code, reason) = match http_code {
        400 => {
            let formatted_message = ERROR_HTML
                .replace("{set error code here owo}", "400 - Bad Request")
                .replace("{set random error message here owo}", &get_random_message(FUNNY_400_MESSAGES));

            (formatted_message.into_bytes(), 400, "Bad Request")
        }
        404 => {
            let formatted_message = ERROR_HTML
                .replace("{set error code here owo}", "404 - Not Found")
                .replace("{set random error message here owo}", &get_random_message(FUNNY_404_MESSAGES));

            (formatted_message.into_bytes(), 404, "Not Found")
        }
        501 => {
            let formatted_message = ERROR_HTML
                .replace("{set error code here owo}", "501 - Not Implemented")
                .replace("{set random error message here owo}", &get_random_message(FUNNY_501_MESSAGES));

            (formatted_message.into_bytes(), 501, "Not Implemented")
        }
        505 => {
            let formatted_message = ERROR_HTML
                .replace("{set error code here owo}", "505 - HTTP Version Not Supported")
                .replace("{set random error message here owo}", &get_random_message(FUNNY_505_MESSAGES));

            (formatted_message.into_bytes(), 505, "HTTP Version Not Supported")
        }
        _ => {
            let formatted_message = ERROR_HTML
                .replace("{set error code here owo}", &http_code.to_string())
                .replace("{set random error message here owo}", "I dont know what this error code is, so just pretend something funny is here :D!");

            (formatted_message.into_bytes(), http_code, "")
        }
    };


    let general_headers: Vec<crate::http::http_1_0_parser::GeneralHeader> = vec![
        crate::http::http_1_0_parser::GeneralHeader::Date({
            let now: DateTime<Utc> = Utc::now();    // Get the current UTC time
            now.format("%a, %d %b %Y %H:%M:%S GMT").to_string() // Format da time as HTTP-date
        }),
        crate::http::http_1_0_parser::GeneralHeader::Pragma("no-cache".to_string()),
    ];

    let response_headers: Vec<crate::http::http_1_0_parser::ResponseHeader> = vec![
        crate::http::http_1_0_parser::ResponseHeader::Server("PolarBear".to_string()),
    ];

    let entity_headers: Vec<EntityHeader> = vec![
        EntityHeader::Allow(vec![GET]),
        EntityHeader::ContentLength(body.len() as u128),
        EntityHeader::ContentType("text/html; charset=utf-8".to_string()),
    ];

    http_1_0_response {
        status_line: StatusLine {
            version: HttpVersion::HTTP_1_0,
            status_code: code,
            reason_phrase: reason.to_string(),
        },
        general_headers,
        response_headers,
        entity_headers,
        body,
    }.into_response()
}

// the world's most original random number generator
fn get_random_message(list: &[&str]) -> String {
    let now = SystemTime::now();
    let duration = match now.duration_since(UNIX_EPOCH) {
        Ok(duration) => duration,
        Err(_) => return String::from("The server failed to generate a random number for a silly message, so you can just watch it suffer without comedic relief."),
    };

    let nanos = duration.as_nanos();

    let index = (nanos % list.len() as u128) as usize;
    list[index].to_string()
}