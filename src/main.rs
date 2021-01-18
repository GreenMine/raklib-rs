use std::{net::UdpSocket, unimplemented};


mod utils;
mod types;
pub mod consts;
mod packets;

use packets::*;

fn main() -> std::io::Result<()> {
    let address = "127.0.0.1:19133";
    let socket = UdpSocket::bind(address)?;
    let mut buffer = [0u8; 2048];

    println!("RakNet connection opened on {}", address);
    println!("Waiting message...");
    loop {
        let (readed_bytes, addr) = socket.recv_from(&mut buffer).expect("no date received!");
        let start = std::time::Instant::now();

        let mut packet = Packet::from_slice(&buffer[..readed_bytes]);

        match packet.id {
            0x1 => {
                let offline_packet = OfflinePingPacket::decode(&mut packet);

                let server_id_string = "MCPE;Rust core test;422;1.16.200;0;2000;2570685482448425430;RakLibRS;Survival;".to_string();
                let reply = OfflinePongPacket::new(offline_packet.time, &server_id_string);

                socket.send_to(&reply.encode().stream.data[..], addr)?;
            },
            0x5 => {
                println!("Open Connection Request 1");

                let request = FirstOpenConnectionRequest::decode(&mut packet);
                let reply = FirstOpenConnectionReply::new(false, request.mtu_lenght);

                socket.send_to(&reply.encode().stream.data[..], addr)?;
                /*println!("Result");
                print_binary(bstream.read_slice(readed_bytes - 1));*/
            },
            0x7 => {
                    println!("Open Connection Request 2");
                    let request2 = SecondOpenConnectionRequest::decode(&mut packet);
                    let reply2 = SecondOpenConnectionReply::new(addr, request2.mtu_length, false);

                    socket.send_to(&reply2.encode().stream.data[..], addr)?;
            }
            _ => {
                print!("Readed data: ");
                print_binary(&buffer[0..readed_bytes]);
                unimplemented!("PACKET ID: 0x{:02X}", packet.id)
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