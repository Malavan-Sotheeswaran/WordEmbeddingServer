use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    collections::HashMap,
    time::{SystemTime, Duration},
};

pub struct Server {
    bind_addr: String,
    key_embed_map: HashMap<String, String>,
    conns: Vec<TcpStream>,
}

impl Server {
    pub fn start(mut self) {
        match TcpListener::bind(&self.bind_addr) {
            Ok(listener) => {
                loop {
                    let start = SystemTime::now();
                    for _ in 1..100 {
                        match listener.accept() {
                            Ok((stream, client)) => {
                                println!("Accepted connection from: {client:?}");
                                self.conns.push(stream);
                            },
                            Err(e) => println!("Error accepting connection: {e:?}")
                        }
                        match start.elapsed() {
                            Ok(elapsed) => if elapsed > Duration::from_millis(1000) { break; },
                            Err(e) => println!("Error checking time spent waiting for conns: {e:?}")
                        }
                    }
                    for stream in &self.conns {
                        let buf_reader = BufReader::new(stream);
                        let received: Vec<_> = buf_reader
                            .lines()
                            .map(|result| result.unwrap())
                            .take_while(|line| !line.is_empty())
                            .collect();

                        println!("Received: {:#?}", received);
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