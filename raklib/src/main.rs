#![feature(type_name_of_val)]

pub mod protocol;
pub mod server;
pub mod utils;

pub use utils::log;

use server::Server;

#[tokio::main]
async fn main() {
    let address = "192.168.1.67:19135".parse().unwrap();

    let mut server = Server::bind(address).await.unwrap();
    server.run().await.unwrap();

    loop {
        if let Some((user, mut stream)) = server.recv().await {
            println!("new user connected: {}", user);
            tokio::spawn(async move {
                loop {
                    if let Some(data) = stream.recv().await {
                        println!("Connected data got(len: {})!", data.len());
                    }
                }
            });
        }
    }
}
