use std::net::{SocketAddr, UdpSocket};

let addrs = [
    SocketAddr::from(([127, 0, 0, 1], 3700)),
    SocketAddr::from(([127, 0, 0, 1], 3701)),
];

let socket = UdpSocket::bind(&addrs[..]).expect("Could not bind to address");
let mut buf = [0; 1024];
let (byte_size, src) = socket.recv_from(&mut buf).expect("Failed to receive data");
let data = &mut buf[..byte_size];

println!("Received {} bytes from {}: {:?}", byte_size, src, data);