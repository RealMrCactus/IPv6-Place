#![allow(non_snake_case)]
use std::net::Ipv6Addr;

use image;
use image::GenericImageView;
use socket2::{Socket, Domain, Type, Protocol};
use std::net::SocketAddrV6;


//const WS: &str    = "wss://ssi.place/ws";
//let image: &str = "out.jpg";

fn process_image(image: &str) {
    let image = image::open(image).unwrap().into_rgb8();
    
    let socket = Socket::new(Domain::IPV6, Type::RAW, Some(Protocol::ICMPV6)).unwrap();
    let payload = [0x80, 0, 0, 0, 0, 0, 0, 0];
    let _ = socket.set_nonblocking(true);
    let _ = socket.set_send_buffer_size(usize::MAX);
    for (x, y, color) in image.enumerate_pixels() {
        // ping format (hex): x, y, r, g, b 2a01:4f8:c012:f8e6:SXXX:YYYY:RR:GGBB s will default to 1
        let ip = Ipv6Addr::new(
            0x2a01, 0x4f8, 0xc012, 0xf8e6, 
            (0x1 << 12) | x as u16, y as u16,
            color[0] as u16, ((color[1] as u16) << 8) | color[2] as u16
        );
        let addr = SocketAddrV6::new(ip, 1, 0, 0);
        
        socket.send_to(&payload, &addr.into()).unwrap();
    }
}
fn main() {
    loop {
        process_image("out.png");
    }
}
