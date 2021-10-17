use http::Request;
use http::Method;
use server::Server;

mod server;
mod http;

fn main() {
    println!("Hello, Server!");

    let server = Server::new("127.0.0.1:8080".to_string());

    server.run();
}





