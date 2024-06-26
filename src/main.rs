use std::io::{BufRead, Read};
use std::net::TcpListener;
use std::thread;
use std::time::Instant;

use artic_tls::tls_handler;

use crate::request_handler::{request_handler, RequestType};

mod response_handler;
mod request_handler;
mod macros;

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
        match connection_listener() {
            Ok(_) => println_cyan!("Polar Bear Stopped!"),
            Err(err) => {
                eprintln_red!("Failed to setup ports. Do you have the necessary permissions?\nError: {:?}", err.root_cause());
                std::process::exit(1);
            }
        };
    });

    //thread::spawn(|| {
    match connection_listener_https() {
        Ok(_) => println_cyan!("Polar Bear Stopped!"),
        Err(err) => {
            eprintln_red!("Failed to setup ports. Do you have the necessary permissions?\nError: {:?}", err.root_cause());
            std::process::exit(1);
        }
    };
    //});

    //println_cyan!("Polar Bear Initialized!");
}

fn connection_listener() -> anyhow::Result<()> {
    let listener = TcpListener::bind("localhost:8080")?;

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let start_time = Instant::now();

                match request_handler(RequestType::Http(stream)) {
                    Ok(_) => println_cyan!("Response Completed in {}μs", (Instant::now() - start_time).as_micros()),
                    Err(e) => eprintln_red!("Failed to handle a request! || {}", e.root_cause())
                }
            }
            Err(e) => eprintln_red!("HTTP Connection failed: {:?}", e),
        }
    }
    Ok(())
}