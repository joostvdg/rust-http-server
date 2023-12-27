
use crate::http::Request;
use std::convert::TryFrom;

use std::io::{Read, Write};
use std::net::TcpListener;

pub(crate) struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Server {
            addr
        }
    }

    pub fn run(self) {
        println!("Listening on {}", self.addr);

        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {
            match listener.accept(){
                Ok((mut stream, client_address)) => {
                    println!("Connection established with {}", client_address);
                    let mut buffer = [0; 1024];

                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Received a request: {}", String::from_utf8_lossy(&buffer));
                            match Request::try_from(&buffer[..]) {
                                Ok(_request) => {},
                                Err(e) => println!("Failed to parse a request: {}", e),
                            }
                        },
                        Err(e) => println!("Failed to read from connection: {}", e),

                    }


                    stream.write(b"Hello World\r\n").expect("Could not respond to client");
                },
                Err(e) => {
                    println!("Failed to establish a connection: {}", e);
                }
            }

        }
    }
}