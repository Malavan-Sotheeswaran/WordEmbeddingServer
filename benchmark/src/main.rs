use std::{
    io::{prelude::*},
    net::{TcpStream},
};

fn main() {
    if let Ok(mut stream) = TcpStream::connect("127.0.0.1:7487") {
        let put = "put key sfpiajfasiofaspfojppspoi\r\n";
        if let Ok(written) = stream.write(put.as_bytes()) {
            println!("Wrote {put:?} to server");
            let mut buf : [u8; 1024] = [0; 1024];
            match stream.read(&mut buf) {
                Ok(bytes_read) => {
                    match std::str::from_utf8(&buf[0..bytes_read]) {
                        Ok(read) => println!("Got {read:?} from server"),
                        Err(e) => println!("Error saving data read from client: {e:?}")
                    }
                }
                Err(e) => {
                    println!("Error reading from connection {e:?}");
                }
            }
        } else {
            println!("Couldn't write to server...");
        }
        let get = "get key\r\n";
        if let Ok(written) = stream.write(get.as_bytes()) {
            println!("Wrote {get:?} to server");
            let mut buf : [u8; 1024*1024] = [0; 1024*1024];
            match stream.read(&mut buf) {
                Ok(bytes_read) => {
                    match std::str::from_utf8(&buf[0..bytes_read]) {
                        Ok(read) => println!("Got {read:?} from server"),
                        Err(e) => println!("Error saving data read from client: {e:?}")
                    }
                }
                Err(e) => {
                    println!("Error reading from connection {e:?}");
                }
            }
        } else {
            println!("Couldn't write to server...");
        }
    } else {
        println!("Couldn't connect to server...");
    }
}
