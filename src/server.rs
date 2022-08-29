use std::{
    io::{prelude::*, ErrorKind},
    net::{TcpListener, TcpStream},
    collections::HashMap,
};

struct Connection {
    pub stream: TcpStream,
    pub buf: String,
}

pub struct Server {
    bind_addr: String,
    key_embed_map: HashMap<String, String>,
    conns: Vec<Connection>,
}

impl Server {
    pub fn start(mut self) {
        match TcpListener::bind(&self.bind_addr) {
            Ok(listener) => {
                listener.set_nonblocking(true).expect("Error setting listener to nonblocking");
                loop {
                    for _ in 1..100 {
                        match listener.accept() {
                            Ok((stream, client)) => {
                                println!("Accepted connection from: {client:?}");
                                self.conns.push(Connection {
                                    stream: stream,
                                    buf: String::new(),
                                });
                            },
                            Err(e) => if e.kind() != ErrorKind::WouldBlock {
                                println!("Error accepting connection: {e:?}")
                            }
                        }
                    }
                    self.conns.retain_mut(|conn| {
                        let mut deleted = false;
                        let mut buf : [u8; 1024] = [0; 1024];
                        match conn.stream.read(&mut buf) {
                            Ok(bytes_read) => if bytes_read == 0 {
                                deleted = true;
                            } else {
                                match std::str::from_utf8(&buf[0..bytes_read]) {
                                    Ok(read) => conn.buf.push_str(read),
                                    Err(e) => println!("Error saving data read from client: {e:?}")
                                }
                            }
                            Err(e) => {
                                println!("Error reading from connection {e:?}");
                                deleted = true;
                            }
                        }
                        !deleted
                    });
                    for conn in &self.conns {
                        let msg = &conn.buf;
                        println!("Got: {msg:?}");
                    }
                }
            }
            Err(e) => println!("Error checking time spent waiting for conns: {e:?}")
        }
    }

    pub fn new(bind_addr: String) -> Server {
        Server {
            bind_addr: bind_addr,
            key_embed_map: HashMap::new(),
            conns: Vec::new(),
        }
    }
}