const MAX_ACCEPTS_PER_CYCLE: usize = 100;
const TEMP_READ_BUF_SIZE: usize = 1024;
const MAX_READ_BUF_SIZE: usize = 4096;

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
                    for _ in 1..MAX_ACCEPTS_PER_CYCLE {
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
                        if conn.buf.len() > MAX_READ_BUF_SIZE {
                            return false;
                        }
                        let mut buf : [u8; TEMP_READ_BUF_SIZE] = [0; TEMP_READ_BUF_SIZE];
                        match conn.stream.read(&mut buf) {
                            Ok(bytes_read) => if bytes_read == 0 {
                                return false;
                            } else {
                                match std::str::from_utf8(&buf[0..bytes_read]) {
                                    Ok(read) => conn.buf.push_str(read),
                                    Err(e) => println!("Error saving data read from client: {e:?}")
                                }
                            }
                            Err(e) => {
                                println!("Error reading from connection {e:?}");
                                return false;
                            }
                        }
                        return true;
                    });
                    for conn in &mut self.conns {
                        if let Some(idx) = conn.buf.find("\r\n") {
                            let mut command: String = conn.buf.drain(..idx+2).collect::<String>();
                            command.truncate(idx);
                            println!("Got command: {command:?}");
                        }
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