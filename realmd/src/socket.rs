

use std::{io, mem};
use std::net::SocketAddr;
use futures::{Future, Poll};
use tokio_core::net::UdpSocket;
use tokio_core::reactor::Core;

use auth_types;

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
                println!("First byte: {}", &self.buf[0]);
                //let mut realPacket: Vec<u8> = self.buf[0..size].to_vec();
                match self.buf[0] {
                    1u8 => {
                            handle_auth_logon_challenge(&self.buf);
                        },
                    _ => {}
                };
                self.to_send = None;
            }

            // If we're here then `to_send` is `None`, so we take a look for the
            // next message we're going to echo back.
            self.to_send = Some(try_nb!(self.socket.recv_from(&mut self.buf)));
        }
    }
}
fn handle_auth_logon_challenge(buf: &Vec<u8>) {
    println!("Header of packet: {:?}", &buf[0..3].to_vec());
    let struct_size = mem::size_of::<auth_types::AUTH_LOGON_CHALLENGE_C>();
    if buf.len() !=  struct_size {
        warn!("Received packet and expected struct are different sizes! {} vs {}", buf.len(), struct_size);
    }
    let  auth_logon_challenge = auth_types::from_packet(&buf);
    println!("packet: {:?}", &auth_logon_challenge);
}
pub fn listen() {
    // Create the event loop that will drive this server
    let mut core = Core::new().unwrap();
    let handle = core.handle();

    // Bind the server's socket
    let addr = "0.0.0.0:3724".parse().unwrap();

    let udp = UdpSocket::bind(&addr, &handle).unwrap();

    info!("Listening UDP");

    core.run(Server {
        socket: udp,
        buf: vec![0; 1024],
        to_send: None,
    }).unwrap();

}