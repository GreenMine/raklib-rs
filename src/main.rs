use std::net::UdpSocket;


mod utils;
mod types;
pub mod consts;
mod packets;

use utils::BinaryStream;
use packets::*;

fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:19133")?;
    let mut buffer = [0u8; 2048];
    
    println!("Waiting message...");
    loop {
        let (readed_bytes, addr) = socket.recv_from(&mut buffer).expect("no date received!");
        let start = std::time::Instant::now();

        /*print!("Result: '");
        &buffer[0..readed_bytes].into_iter().for_each(|&d| print!("{}", d as char));
        println!("'");*/
        /*println!("Got:");
        print_binary(&buffer[0..readed_bytes]);*/

        let mut bstream = BinaryStream::from_slice(&buffer[..readed_bytes]);
        let packet_id = bstream.read::<u8>();
        match packet_id {
            0x1 => {
                let offline_packet = OfflinePingPacket::decode(&mut bstream);

                let server_id_string = "MCPE;Rust core test;422;1.16.200;0;2000;2570685482448425430;RakLibRS;Survival;".to_string();
                let response = OfflinePongPacket::new(offline_packet.time, server_id_string);

                socket.send_to(&response.encode().stream.data[..], addr)?;
            },
            0x5 => {
                println!("Open Connection Reply 1");
                let mut response = BinaryStream::with_len(1 + 16 + 8 + 1 + 2);
                response.add(0x06_u8);
                response.add_magic(consts::MAGIC);
                response.add(consts::SERVER_GUID);
                response.add(false);
                response.add(readed_bytes as u16);

                socket.send_to(&response.data[..], addr)?;
                /*println!("Result");
                print_binary(bstream.read_slice(readed_bytes - 1));*/
            },
            0x7 => {
                    println!("Open Connection Request 2");

                    let mut response = BinaryStream::with_len(1 + 16 + 8 + 7 + 2 + 1);

                    
            }
            _ => {
                unimplemented!("PACKET ID: 0x{:02X}", packet_id)
            }
        }

        println!("One packet execution time: {}micros.", start.elapsed().as_micros());
    }
    Ok(())
}

fn print_binary(bin: &[u8]) {
        bin.iter().enumerate().for_each(|(i, &b)| {
            print!("0x{:02X} ", b);
            if (i + 1) % 15 == 0 {
                println!();
            }
        });
        println!();
}