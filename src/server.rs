use std::{
    io::{prelude::*, ErrorKind},
    net::{TcpListener, TcpStream},
    collections::HashMap,
    time::{SystemTime, Duration},
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
                listener.set_nonblocking(true);
                loop {
                    let start = SystemTime::now();
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
                        match start.elapsed() {
                            Ok(elapsed) => if elapsed > Duration::from_millis(1000) { break; },
                            Err(e) => println!("Error checking time spent waiting for conns: {e:?}")
                        }
                    }
                    self.conns.retain_mut(|conn| {
                        let mut deleted = false;
                        match conn.stream.read_to_string(&mut conn.buf) {
                            Ok(bytes_read) => if bytes_read == 0 {
                                deleted = true;
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