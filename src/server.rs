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
                //main event loop
                loop {
                    //accept new clients
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

                    //check client state
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

                    //check client buffers for commands
                    for conn in &mut self.conns {
                        if let Some(idx) = conn.buf.find("\r\n") {
                            let mut full_command: String = conn.buf.drain(..idx+2).collect::<String>();
                            full_command.truncate(idx);
                            if let Some((command,key)) = full_command.split_once(' ') {
                                match command {
                                    "put" => {
                                        if let Some((key, rest)) = key.split_once(' ') {
                                            println!("Request to set {key:?} to {rest:?}");
                                            self.key_embed_map.insert(String::from(key), String::from(rest));
                                        }
                                    },
                                    "get" => {
                                        println!("Request for {key:?}");
                                        match self.key_embed_map.get(key) {
                                            Some(embed) => {
                                                match conn.stream.write(embed.as_bytes()) {
                                                    Ok(_writen) => {
        
                                                    },
                                                    Err(e) => println!("Error writing to connection {e:?}")
                                                }
                                            }
                                            None => {
                                                match conn.stream.write("(none)".as_bytes()) {
                                                    Ok(_writen) => {
        
                                                    },
                                                    Err(e) => println!("Error writing to connection {e:?}")
                                                }
                                            }
                                        }
                                    },
                                    "del" => {
                                        println!("Request to delete {key:?}");
                                        self.key_embed_map.remove(key);
                                    },
                                    _ => {
                                        println!("Bad command: {command:?}");
                                    }
                                }
                            } else {
                                println!("Invalid input: {full_command:?}");
                            }
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