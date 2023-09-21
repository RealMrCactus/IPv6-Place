#![allow(non_snake_case)]
use std::net::Ipv6Addr;

use image;
use image::GenericImageView;
use socket2::{Socket, Domain, Type, Protocol};
use std::net::{SocketAddrV6};


//const WS: &str    = "wss://ssi.place/ws";
//let image: &str = "out.jpg";

fn process_image(image: &str) {
    let image = image::open("src/out.jpg").unwrap();
    
    let socket = Socket::new(Domain::IPV6, Type::RAW, Some(Protocol::ICMPV6)).unwrap();
    let payload = [0; 8];
    loop {
        for (x, y, color) in image.pixels() {
            // ping format (hex): x, y, r, g, b 2a01:4f8:c012:f8e6:SXXX:YYYY:RR:GGBB s will default to 1
            let ip = Ipv6Addr::new(
                0x2a01, 0x4f8, 0xc012, 0xf8e6, 
                (0x2 << 12) | x as u16, y as u16,
                color[0] as u16, ((color[1] as u16) << 8) | color[2] as u16
            );
            let addr = SocketAddrV6::new(ip, 1, 0, 0);
            socket.send_to(&payload, &addr.into()).unwrap();
        }
    }
    
}

fn main() {
    // ping("2a01:4f8:c012:f8e6:f1f9:1c6:32:0000");
    
    process_image("src/out.jpg");
}
