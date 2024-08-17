use std::io::{BufRead, BufReader, Read, Write};
use std::net::TcpListener;
use std::thread;

use artic_tls::tls_handler;

mod response_handler;
mod request_handler;
mod macros;
mod http_parser;
//fn connection_listener_https() -> anyhow::Result<()> {
//    let cert_file = fs::read(Path::new("cert.pem"))?;
//    let key_file = fs::read(Path::new("key.pem"))?;
//
//    let identity = Identity::from_pkcs8(&cert_file, &key_file)?;
//    let acceptor = TlsAcceptor::builder(identity).build()?;
//
//    let acceptor = Arc::new(acceptor);
//    let listener = TcpListener::bind("localhost:443")?;
//
//    for stream in listener.incoming() {
//        match stream {
//            Ok(stream) => {
//                let start_time = Instant::now();
//                let acceptor = acceptor.clone();
//                match acceptor.accept(stream) {
//                    Ok(stream) => {
//                        match request_handler(RequestType::Https(stream)) {
//                            Ok(_) => println_cyan!("Response Completed in {}μs", (Instant::now() - start_time).as_micros()),
//                            Err(err) => eprintln_red!("Failed to handle client: {}", err.root_cause())
//                        }
//                    }
//                    Err(err) => eprintln_red!("Error while connecting with HTTPS || Err: {err}"),
//                };
//            }
//            Err(err) => {
//                eprintln_red!("HTTPS Connection failed! || Err: {err}");
//            }
//        }
//    }
//    Ok(())
//}


fn connection_listener_https() -> anyhow::Result<()> {
    let listener = TcpListener::bind("localhost:4443")?;

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let mut buffer = Vec::new();
                stream.read_to_end(&mut buffer)?;

                tls_handler(buffer);
            }
            Err(e) => eprintln_red!("HTTP Connection failed: {:?}", e),
        }
    }
    Ok(())
}


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

                // 8 bits
                //let mut buffer = Vec::new();
                //stream.read_to_end(&mut buffer).unwrap_or_default();

                //let mut buffer = Vec::new();
                //let bytes_read = stream.read_to_end(&mut buffer)?;

                let mut buf_reader = BufReader::new(&mut stream);

                let mut buffer: String = String::new();

                loop {
                    // check if the current buffer contains /r/n/r/n (which indicates that the http header is done)

                    if buffer.contains("\r\n\r\n") {
                        // stawp reading
                        break;
                    }

                    match buf_reader.read_line(&mut buffer) {
                        Ok(0) => {
                            // If read_line returns 0, it means the stream has reached EOF
                            println!("Connection closed.");

                            //// browser doesn't like playing ping pong? hmm

                            break;
                        }
                        Err(e) => {
                            // Handle the error.
                            eprintln!("Error reading from stream: {}", e);
                            break;
                        }
                        _ => {}
                    }
                }

                print!("{}", buffer);


                if buffer.is_empty() {
                    eprintln_red!("that is an uh.. empty request? what do ya want son? || {:?}", stream);
                    //stream.shutdown(Shutdown::Both).unwrap_or_default();
                } else {

                    //
                    //
                    //    match request_handler(RequestType::Http(stream)) {
                    //        Ok(_) => println_cyan!("Response Completed in {}μs", (Instant::now() - start_time).as_micros()),
                    //        Err(e) => eprintln_red!("Failed to handle a request! || {}", e.root_cause())
                    //    }
                }

                //match request_handler(RequestType::Http(stream)) {
                //    Ok(_) => println_cyan!("Response Completed in {}μs", (Instant::now() - start_time).as_micros()),
                //    Err(e) => eprintln_red!("Failed to handle a request! || {}", e.root_cause())
                //}
            }
            Err(e) => eprintln_red!("HTTP Connection failed: {:?}", e),
        }
    }
    Ok(())
}