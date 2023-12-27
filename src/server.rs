
use crate::http::{ParseError, Request, Response, StatusCode};
use std::convert::TryFrom;

use std::io::{Read, Write};
use std::net::TcpListener;

pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response;
    fn handle_bad_request(&mut self, e: &ParseError) -> Response {
        println!("Failed to parse a request: {}", e);
        Response::new(StatusCode::BadRequest, Some("<h1>400</h1><p>Bad Request</p>".to_string()))
    }
}

pub(crate) struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Server {
            addr
        }
    }

    pub fn run(self, mut handler: impl Handler){
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
                            let response =  match Request::try_from(&buffer[..]) {
                                Ok(_request) => {
                                    handler.handle_request(&_request)
                                },
                                Err(e) => {
                                    println!("Failed to parse a request: {}", e);
                                    handler.handle_bad_request(&e)
                                }
                            };
                            if let Err(e) = response.send(&mut stream) {
                                println!("Failed to send a response: {}", e);
                            }
                        },
                        Err(e) => println!("Failed to read from connection: {}", e),

                    }

                },
                Err(e) => {
                    println!("Failed to establish a connection: {}", e);
                }
            }

        }
    }
}