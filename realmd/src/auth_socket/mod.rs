use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

use simplelog::LogLevel::*;

mod auth_logon_challenge;

pub fn listen(pool: mysql::Pool) {
    let bind_address: &str = "127.0.0.1:3724";
    info!("Opening AuthSocket on {}", bind_address);
    let listener = TcpListener::bind(bind_address).unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let mut buf: Vec<u8> = Vec::new();
                let mut resp: Vec<u8> = Vec::new();
                let bytes_read = stream.read_to_end(&mut buf);
                match bytes_read {
                    Ok(byte_count) => {
                        println!(
                            "read {} bytes from {:?}",
                            byte_count,
                            stream.peer_addr().unwrap()
                        );
                        if (byte_count == 0) {}

                        resp = match auth_logon_challenge::auth_types::AuthCmds::from(&buf[0]) {
                            auth_logon_challenge::auth_types::AuthCmds::LogonChallenge => {
                                auth_logon_challenge::handleAuthLogonChallenge(buf, pool.clone())
                            }
                            auth_logon_challenge::auth_types::AuthCmds::LogonProof => resp,
                            _ => {
                                println!("Got unknown cmd!");
                                resp
                            }
                        }
                    }
                    Err(err) => eprintln!("{}", err),
                }

                stream.write(&resp).expect("Response failed");
            }
            Err(e) => {
                println!("Unable to connect: {}", e);
            }
        }
    }
}
