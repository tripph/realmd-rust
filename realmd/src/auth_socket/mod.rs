mod auth_types;

use std::net::{TcpStream, TcpListener};
use std::io::{Write, Read};
use simplelog::LogLevel::*;
pub fn listen(pool: mysql::Pool) {
    info!("Opening AuthSocket on 127.0.0.1:3724");
    let listener = TcpListener::bind("127.0.0.1:3724").unwrap();
//    let pool = my::Pool::new("mysql://root:root@localhost:3306/realmd").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let mut buf = [0; 256];
                let bytes_read = stream.read(&mut buf);
                match bytes_read {
                    Ok(byte_count) => {
                        println!("read {} bytes.", byte_count);
                        if (byte_count == 0) {}
                    }
                    Err(err) => { eprintln!("{}", err) }
                }

                match auth_types::AuthCmds::from_u8(buf[0]) {
                    auth_types::AuthCmds::LogonChallenge => {
                        println!("Got cmd 0! Packet length: {}", buf.len());
                        let header = auth_types::getHeader(buf);
                        let challenge = auth_types::getLogonChallenge(buf, header);
                        println!("challenge: {}", challenge);
                    }
                    auth_types::AuthCmds::LogonProof => {}
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


