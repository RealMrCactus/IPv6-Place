#![allow(non_snake_case)]

use tungstenite::{connect, Message};
use url::Url;

fn ws_test() {
    env_logger::init();

    let (mut socket, response) =
        connect(Url::parse("wss://ssi.place/ws").unwrap()).expect("Can't connect");

    println!("Connected to the server");
    println!("Response HTTP code: {}", response.status());
    println!("Response contains the following headers:");
    for (ref header, _value) in response.headers() {
        println!("* {}", header);
    }

    socket.send(Message::Text("Hello WebSocket".into())).unwrap();
    loop {
        let msg = socket.read().expect("Error reading message");
        println!("Received: {}", msg);
    }
    // socket.close(None);
}

fn main() {
    //ws_test();
    println!("\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n>> :");
}
