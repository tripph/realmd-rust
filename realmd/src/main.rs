use std::net::{TcpStream, TcpListener};
use std::io::{Write,Read};
mod auth_types;
mod auth_codes;
fn main() {
    let listener = TcpListener::bind("127.0.0.1:3724").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let mut buf = vec![0; 0];
                stream.read_to_end(&mut buf);
                match auth_codes::AuthCmds::from_u8(buf[0]) {
                    auth_codes::AuthCmds::LogonChallenge => {
                        println!("Got cmd 0! Packet length: {}", buf.len());
                        let header = auth_types::getHeader(&buf);
                        let challenge = auth_types::getLogonChallenge(&buf, header);
                        println!("challenge: {}", challenge);
                    },
                    auth_codes::AuthCmds::LogonProof => {

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