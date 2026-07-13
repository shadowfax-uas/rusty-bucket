use std::{
    io,
    net::{SocketAddr, UdpSocket},
    str,
};

pub(crate) fn listen() -> io::Result<()> {
    let addrs = [
        SocketAddr::from(([127, 0, 0, 1], 3700)),
        SocketAddr::from(([127, 0, 0, 1], 3701)),
    ];

    let socket = UdpSocket::bind(&addrs[..]).expect("Could not bind to address");
    let mut buf = [0_u8; 2048];

    loop {
        let (len, src) = socket.recv_from(&mut buf)?;
        let bytes = &buf[..len];
        println!("Received {len} bytes from {src}: {bytes:02x?}");

        match str::from_utf8(bytes) {
            Ok(s) => println!("Received string: {s:?}"),
            Err(_) => println!("utf8=<not valid utf-8>"),
        }
    }
}
