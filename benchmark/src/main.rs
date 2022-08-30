use std::{
    io::{prelude::*},
    net::{TcpStream},
};

fn main() {
    if let Ok(mut stream) = TcpStream::connect("127.0.0.1:7487") {
        let string = "get key\r\n";
        loop {
            if let Ok(written) = stream.write(string.as_bytes()) {
                println!("Wrote {string:?} to server");
                let mut buf : [u8; 1024] = [0; 1024];
                match stream.read(&mut buf) {
                    Ok(bytes_read) => {
                        println!("Got {buf:?} from server")
                    }
                    Err(e) => {
                        println!("Error reading from connection {e:?}");
                    }
                }
            } else {
                println!("Couldn't write to server...");
            }
            std::thread::sleep(std::time::Duration::from_millis(1000));
        }
    } else {
        println!("Couldn't connect to server...");
    }
}
