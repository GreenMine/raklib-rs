use raklib::client;
use std::net::ToSocketAddrs;

const SERVER_ADDR: &'static str = "mps.mc-complex.com:19132";

#[tokio::main]
async fn main() {
    env_logger::init();

    let pong = client::ping(SERVER_ADDR).await.unwrap();

    println!("Pong info:");
    println!("Time: {}", pong.time);
    println!("Server id: {}", pong.server_id_string);
}
