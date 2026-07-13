mod udp_listener;

use std::io;
use udp_listener::listen;

fn main() -> io::Result<()> {
    listen()
}
