use std::{
    io::{prelude::*},
    net::{TcpStream},
};

fn main() {
    if let Ok(mut stream) = TcpStream::connect("127.0.0.1:7487") {
        let string = "Hi!\r\n";
        loop {
            if let Ok(written) = stream.write(string.as_bytes()) {
                println!("Wrote {written:?} to server");
            } else {
                println!("Couldn't write to server...");
            }
            std::thread::sleep(std::time::Duration::from_millis(1000));
        }
    } else {
        println!("Couldn't connect to server...");
    }
}
