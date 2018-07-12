

extern crate futures;
extern crate tokio_core;
extern crate tokio_io;

//use std::env;
//use std::net::SocketAddr;

use futures::Future;
use futures::stream::Stream;
use tokio_io::AsyncRead;
use tokio_io::io::copy;
use tokio_core::net::TcpListener;
use tokio_core::reactor::Core;

use std::env;
use std::net::SocketAddr;

use std::mem;
use self::handle_auth_logon_challenge;

//struct Server {
//    socket: UdpSocket,
//    buf: Vec<u8>,
//    to_send: Option<(usize, SocketAddr)>,
//}
//
//impl Future for Server {
//    type Item = ();
//    type Error = io::Error;
//
//    fn poll(&mut self) -> Poll<(), io::Error> {
//        loop {
//            // First we check to see if there's a message we need to echo back.
//            // If so then we try to send it back to the original source, waiting
//            // until it's writable and we're able to do so.
//            if let Some((size, peer)) = self.to_send {
//                let amt = try_nb!(self.socket.send_to(&self.buf[..size], &peer));
//                println!("Echoed {}/{} bytes to {}", amt, size, peer);
//                println!("First byte: {}", &self.buf[0]);
//                //let mut realPacket: Vec<u8> = self.buf[0..size].to_vec();
//                match self.buf[0] {
//                    1u8 => {
//                            handle_auth_logon_challenge(&self.buf);
//                        },
//                    _ => {}
//                };
//                self.to_send = None;
//            }
//
//            // If we're here then `to_send` is `None`, so we take a look for the
//            // next message we're going to echo back.
//            self.to_send = Some(try_nb!(self.socket.recv_from(&mut self.buf)));
//        }
//    }
//}
fn handle_auth_logon_challenge(buf: &Vec<u8>) {
    println!("Header of packet: {:?}", &buf[0..3].to_vec());
    let struct_size = mem::size_of::<auth_types::AUTH_LOGON_CHALLENGE_C>();
    if buf.len() !=  struct_size {
        warn!("Received packet and expected struct are different sizes! {} vs {}", buf.len(), struct_size);
    }
    let  auth_logon_challenge = auth_types::from_packet(&buf);
    println!("packet: {}", &auth_logon_challenge);
}
pub fn listen() {
    // Create the event loop that will drive this server
    let mut core = Core::new().unwrap();
    let handle = core.handle();

    // Bind the server's socket
    let addr = "0.0.0.0:3724".parse().unwrap();

    let socket = TcpListener::bind(&addr, &handle).unwrap();

    info!("Listening TCP");

    let done = socket.incoming().for_each(move |(socket, addr)| {

        // Once we're inside this closure this represents an accepted client
        // from our server. The `socket` is the client connection and `addr` is
        // the remote address of the client (similar to how the standard library
        // operates).
        //
        // We just want to copy all data read from the socket back onto the
        // socket itself (e.g. "echo"). We can use the standard `io::copy`
        // combinator in the `tokio-core` crate to do precisely this!
        //
        // The `copy` function takes two arguments, where to read from and where
        // to write to. We only have one argument, though, with `socket`.
        // Luckily there's a method, `Io::split`, which will split an Read/Write
        // stream into its two halves. This operation allows us to work with
        // each stream independently, such as pass them as two arguments to the
        // `copy` function.
        //
        // The `copy` function then returns a future, and this future will be
        // resolved when the copying operation is complete, resolving to the
        // amount of data that was copied.
        let (reader, writer) = socket.split();
//        let amt = copy(reader, writer);
//
//        // After our copy operation is complete we just print out some helpful
//        // information.
//        let msg = amt.then(move |result| {
//            match result {
//                Ok((amt, _, _)) => println!("wrote {} bytes to {}", amt, addr),
//                Err(e) => println!("error on {}: {}", addr, e),
//            }
//
//            Ok(())
//        });

        // And this is where much of the magic of this server happens. We
        // crucially want all clients to make progress concurrently, rather than
        // blocking one on completion of another. To achieve this we use the
        // `spawn` function on `Handle` to essentially execute some work in the
        // background.
        //
        // This function will transfer ownership of the future (`msg` in this
        // case) to the event loop that `handle` points to. The event loop will
        // then drive the future to completion.
        //
        // Essentially here we're spawning a new task to run concurrently, which
        // will allow all of our clients to be processed concurrently.
        handle.spawn(msg);

        Ok(())
    });

    core.run(done).unwrap();

}