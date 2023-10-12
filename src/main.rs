#![allow(non_snake_case)]
use std::{net::Ipv6Addr, process::exit};
use std::time;
use clap::Parser;
use spin_sleep;
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
    image: String,
    #[arg(short, long)]
    targetpps: u64
}

fn make_address(size: u8, x: u16, y: u16, r: u8, g: u8, b: u8) -> Ipv6Addr {
    Ipv6Addr::new(
        0x2A01, 0x04F8, 0xC012, 0xF8E6,
		((size as u16) * 0x1000) + x, y, r as u16, ((g as u16) << 8) | (b as u16)
    )
}

fn build_addresses(image: String) -> Vec<SocketAddrV6> {
    let mut addrs = Vec::new();
    let image = image::open(image).unwrap().into_rgb8();
    for (x, y, color) in image.enumerate_pixels() {
        let addr = SocketAddrV6::new(
            make_address(1, x as u16, y as u16, color[0], color[1], color[2]),
            0,
            0,
            0
        );
        addrs.push(addr);
    }
    addrs
}

fn main() {
    std::env::set_var("RUST_BACKTRACE", "1");
    let args = Args::parse();
        
    let addr_list = build_addresses(args.image);
    let socket = Socket::new(Domain::IPV6, Type::RAW, Some(Protocol::ICMPV6)).unwrap();
    _=socket.set_nonblocking(true);
    _=socket.set_send_buffer_size(1024 * 1024 * 64);

    let delay = 1_000_000_000 / args.targetpps;
    println!("Target PPS is {}, using {}ns delay", args.targetpps, delay);
    // It isn't a good idea to set the sleep accuracy to 100ms, but we'll barely use the native sleep soo
    let spin_sleeper = spin_sleep::SpinSleeper::new(100_000_000).with_spin_strategy(spin_sleep::SpinStrategy::SpinLoopHint);
    let payload = [0x80, 0, 0, 0, 0, 0, 0, 0];
    loop {
        for addr in addr_list.iter().cloned() {
           // println!("{}",addr);
            socket.send_to(&payload, &addr.into()).ok();
            spin_sleeper.sleep_ns(delay);
        }
    }
}
