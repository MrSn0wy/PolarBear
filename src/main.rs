use std::io::{BufRead, BufReader, Read, Write};
use std::net::TcpListener;
use std::thread;

use http::http_init::parser;

use crate::http::http_0_9_parser;
use crate::http::http_0_9_parser::http_0_9_request;
use crate::http::http_code_handler::handle_error_codes;
use crate::http::http_init::HttpVersion;
use crate::needed::Polar;

mod needed;

mod http;
mod content_fetcher;
//#[path= "http/http.rs"]
//mod http;

//fn connection_listener_https() -> anyhow::Result<()> {
//    let listener = TcpListener::bind("localhost:4443")?;
//
//    for stream in listener.incoming() {
//        match stream {
//            Ok(mut stream) => {
//                let mut buffer = Vec::new();
//                stream.read_to_end(&mut buffer)?;
//
//                tls_handler(buffer);
//            }
//            Err(e) => eprintln_red!("HTTP Connection failed: {:?}", e),
//        }
//    }
//    Ok(())
//}

fn main() {
    println_cyan!("Initializing Polar Bear!");

    thread::spawn(|| {
        match connection_listener("localhost:4443".to_string()) {
            Ok(_) => println_cyan!("Polar Bear Stopped!"),
            Err(err) => {
                eprintln_red!("Failed to setup ports. Do you have the necessary permissions?\nError: {:?}", err.root_cause());
                std::process::exit(1);
            }
        };
    });

    //thread::spawn(|| {
    match connection_listener("localhost:8080".to_string()) {
        Ok(_) => println_cyan!("Polar Bear Stopped!"),
        Err(err) => {
            eprintln_red!("Failed to setup ports. Do you have the necessary permissions?\nError: {:?}", err.root_cause());
            std::process::exit(1);
        }
    };
    //});

    //println_cyan!("Polar Bear Initialized!");
}

fn connection_listener(addr: String) -> anyhow::Result<()> {
    let listener = TcpListener::bind(addr)?;

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                //let start_time = Instant::now();

                let mut buf_reader = BufReader::new(&mut stream);

                let mut buffer: Vec<String> = vec![];

                loop {
                    let mut temp_buffer: String = String::new();

                    match buf_reader.read_line(&mut temp_buffer) {
                        Ok(0) => {
                            // If read_line returns 0, it means the stream has reached EOF
                            println!("\nConnection closed.");

                            //// browser doesn't like playing ping pong? hmm
                            break;
                        }
                        Ok(_) => {
                            // check if the current buffer contains /r/n/r/n (which indicates that the http header is done)
                            if temp_buffer == "\r\n" {
                                // stawp reading
                                buffer.push(temp_buffer);
                                break;
                            }

                            buffer.push(temp_buffer);
                        }
                        Err(e) => {
                            // Handle the error.
                            eprintln!("Error reading from stream: {}", e);
                            break;
                        }
                    }
                }


                if buffer.is_empty() {
                    eprintln_red!("that is an uh.. empty request? what do ya want son? || {:?}\n", stream);
                    //stream.shutdown(Shutdown::Both).unwrap_or_default();
                } else {
                    println!("{:?}", buffer);

                    // does the parsing, if it isn't successful we just give an error page!
                    match parser(buffer.first().unwrap_or(&String::new())) {
                        Polar::Silly(error_code) => {
                            let html = handle_error_codes(error_code);

                            stream.write_all(&html)?;
                            stream.flush()?;
                        }

                        Polar::Some(request_line) => {
                            println_cyan!("output: [{:?} | {:?} | {:?}]", request_line.method, request_line.uri, request_line.version);

                            match request_line.version {

                                //HttpVersion::HTTP_0_9 => {}
                                //HttpVersion::HTTP_1_0 => {}
                                //HttpVersion::HTTP_1_1 => {}
                                //HttpVersion::HTTP_2_0 => {}
                                HttpVersion::HTTP_3_0 => {}
                                _ => {
                                    let request = http_0_9_request {
                                        request_line,
                                    };

                                    let response = http_0_9_parser::give_response(request);

                                    match response {
                                        Polar::Some(response) => {
                                            stream.write_all(&response.body)?;
                                            stream.flush()?;
                                        }
                                        Polar::Silly(error_code) => {
                                            let html = handle_error_codes(error_code);

                                            stream.write_all(&html)?;
                                            stream.flush()?;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            Err(e) => eprintln_red!("HTTP Connection failed: {:?}", e),
        }
    }
    Ok(())
}