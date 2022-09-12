const MAX_ACCEPTS_PER_CYCLE: usize = 100;
const TEMP_READ_BUF_SIZE: usize = 1024;
const MAX_READ_BUF_SIZE: usize = 4096;

use std::{
    io::{prelude::*, ErrorKind},
    net::{TcpListener, TcpStream},
    collections::HashMap,
};

use rust_bert::pipelines::sentence_embeddings::{SentenceEmbeddingsBuilder, SentenceEmbeddingsModel, SentenceEmbeddingsModelType};

struct Connection {
    pub stream: TcpStream,
    pub buf: String,
}

pub struct Server {
    bind_addr: String,
    key_embed_map: HashMap<String, String>,
    conns: Vec<Connection>,
    model: SentenceEmbeddingsModel,
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
                                            match self.model.encode(rest) {
                                                Ok(embedding) => {
                                                    self.key_embed_map.insert(String::from(key), format!("embedding:?"));
                                                },
                                                Err(e) => {
                                                    println!("Failed computing embedding {e:?}");
                                                    match conn.stream.write("FAIL\r\n".as_bytes()) {
                                                        Ok(_writen) => {
            
                                                        },
                                                        Err(e) => println!("Error writing to connection {e:?}")
                                                    }
                                                }
                                            }
                                            match conn.stream.write("OK\r\n".as_bytes()) {
                                                Ok(_writen) => {
    
                                                },
                                                Err(e) => println!("Error writing to connection {e:?}")
                                            }
                                        }
                                    },
                                    "get" => {
                                        println!("Request for {key:?}");
                                        match self.key_embed_map.get(key) {
                                            Some(embed) => {
                                                match conn.stream.write(embed.as_bytes()) {
                                                    Ok(_writen) => {
                                                        match conn.stream.write("\r\n".as_bytes()) {
                                                            Ok(_writen) => {
                
                                                            },
                                                            Err(e) => println!("Error writing to connection {e:?}")
                                                        }
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
                                        match conn.stream.write("OK\r\n".as_bytes()) {
                                            Ok(_writen) => {

                                            },
                                            Err(e) => println!("Error writing to connection {e:?}")
                                        }
                                    },
                                    _ => {
                                        println!("Unknown command: {command:?}");
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

    pub fn new(bind_addr: String) -> Result<Server, rust_bert::RustBertError> {
        let model = SentenceEmbeddingsBuilder::remote(SentenceEmbeddingsModelType::DistiluseBaseMultilingualCased)
        .with_device(tch::Device::cuda_if_available())
        .create_model()?;
        Ok(Server {
            bind_addr: bind_addr,
            key_embed_map: HashMap::new(),
            conns: Vec::new(),
            model: model,
        })
    }
}