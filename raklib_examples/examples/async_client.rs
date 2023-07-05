use raklib::client;
use std::net::ToSocketAddrs;

const SERVER_ADDR: &'static str = "127.0.0.1:19135";

#[tokio::main]
async fn main() {
    env_logger::init();

    let pong = client::ping(SERVER_ADDR).await.unwrap();

    println!("Pong info:");
    println!("Time: {}", pong.time);
    println!("Server id: {}", pong.server_id_string);
}
