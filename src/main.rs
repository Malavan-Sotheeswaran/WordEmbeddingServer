const NUM_THREADS: usize = 4;
pub mod server;
use std::sync::{Arc, RwLock};
use std::thread;

fn main() {
    let data = Arc::new(RwLock::new(server::ServerData::new()));
    let mut handles = vec![];

    for _ in 1..NUM_THREADS {
        let server = server::ServerThread::new("127.0.0.1:7487".to_string(), Arc::clone(&data))
            .expect("Could not create rust_burt model");
        let handle = thread::spawn(move || {
            server.start();
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
