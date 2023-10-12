#![allow(non_snake_case)]
use std::net::Ipv6Addr;

use clap::Parser;
use image;
use image::GenericImageView;
use socket2::{Domain, Protocol, Socket, Type};
use std::net::SocketAddrV6;

//const WS: &str    = "wss://ssi.place/ws";
//let image: &str = "out.jpg";

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    image: String
}

fn build_addresses(image: String) -> Vec<SocketAddrV6> {
    let mut addrs = Vec::new();
    let image = image::open(image).unwrap().into_rgb8();
    for (x, y, color) in image.enumerate_pixels() {
        // ping format (hex): x, y, r, g, b 2a01:4f8:c012:f8e6:SXXX:YYYY:RR:GGBB s will default to 1
        let ip = Ipv6Addr::new(
            0x2a01,
            0x4f8,
            0xc012,
            0xf8e6,
            (0x1 << 12) | x as u16,
            y as u16,
            color[0] as u16,
            ((color[1] as u16) << 8) | color[2] as u16,
        );
        let addr = SocketAddrV6::new(ip, 0, 0, 0);
        addrs.push(addr);
    }
    addrs
}

fn main() {
    std::env::set_var("RUST_BACKTRACE", "1");
    let args = Args::parse();
        
    let addr_list = build_addresses(args.image);
    let socket = Socket::new(Domain::IPV6, Type::RAW, Some(Protocol::ICMPV6)).unwrap();
    let payload = [0x80, 0, 0, 0, 0, 0, 0, 0];
    let _ = socket.set_nonblocking(true);
    let _ = socket.set_send_buffer_size(1024 * 1024 * 64);
    loop {
        for addr in addr_list.iter().cloned() {
           // println!("{}",addr);
            socket.send_to(&payload, &addr.into()).unwrap();
        }
    
    }
}
