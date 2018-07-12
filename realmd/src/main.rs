use std::net::{TcpStream, TcpListener};
use std::io::{Write,Read};
mod auth_types;
mod auth_codes;
fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let mut buf = vec![0; 300];
                stream.read(&mut buf);
                match buf[0] {
                    auth_codes::AuthCmds::LogonChallenge => {
                        println!("Got cmd 0! Packet length: {}", buf.len());
                        let auth = auth_types::from_packet(&buf);
                        println!("auth: {}", auth);
                    }
                    0x01 => {

                    }
                    _ => {
                        println!("Got unknown cmd!");
                    }
                }
                stream.write(&buf).expect("Response failed");
            }
            Err(e) => {
                println!("Unable to connect: {}", e);
            }
        }
    }
}