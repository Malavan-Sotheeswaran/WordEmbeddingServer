pub mod server;

fn main() {
    let server = server::Server::new("127.0.0.1:7487".to_string());
    server.start();
}