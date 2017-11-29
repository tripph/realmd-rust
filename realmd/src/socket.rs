

use std::{env, io};
use std::net::SocketAddr;

use futures::{Future, Poll};
use tokio_core::net::UdpSocket;
use tokio_core::reactor::Core;
struct Server {
    socket: UdpSocket,
    buf: Vec<u8>,
    to_send: Option<(usize, SocketAddr)>,
}

impl Future for Server {
    type Item = ();
    type Error = io::Error;

    fn poll(&mut self) -> Poll<(), io::Error> {
        loop {
            // First we check to see if there's a message we need to echo back.
            // If so then we try to send it back to the original source, waiting
            // until it's writable and we're able to do so.
            if let Some((size, peer)) = self.to_send {
                let amt = try_nb!(self.socket.send_to(&self.buf[..size], &peer));
                println!("Echoed {}/{} bytes to {}", amt, size, peer);
                self.socket.send(&self.buf);
                self.to_send = None;
            }

            // If we're here then `to_send` is `None`, so we take a look for the
            // next message we're going to echo back.
            self.to_send = Some(try_nb!(self.socket.recv_from(&mut self.buf)));
        }
    }
}
pub fn listen() {
    // Create the event loop that will drive this server
    let mut core = Core::new().unwrap();
    let handle = core.handle();

    // Bind the server's socket
    let addr = "192.168.1.18:12345".parse().unwrap();

    let udp = UdpSocket::bind(&addr, &handle).unwrap();

    info!("Listening UDP");

    core.run(Server {
        socket: udp,
        buf: vec![0; 1024],
        to_send: None,
    }).unwrap();

}