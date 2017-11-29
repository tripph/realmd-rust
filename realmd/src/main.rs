#[macro_use] extern crate log;
extern crate simplelog;
extern crate chrono;
use chrono::{Utc};
use simplelog::*;

use std::fs;
use std::fs::File;
mod auth_types;

fn setup_logging() {
    match fs::create_dir("logs") {
        Err(why) => println!("! {:?}", why.kind()),
        Ok(_) => println!("created logs dir!"),
    }
    let current_time = Utc::now();
    let log_file = format!("logs/realmd-{}.log", current_time.timestamp());
    CombinedLogger::init(vec![
        TermLogger::new(LogLevelFilter::Warn, Config::default()).unwrap(),
        TermLogger::new(LogLevelFilter::Debug, Config::default()).unwrap(),
        WriteLogger::new(LogLevelFilter::Info, Config::default(), File::create(&log_file).unwrap()),
        WriteLogger::new(LogLevelFilter::Error, Config::default(), File::create(&log_file).unwrap()),
     ]
    ).unwrap();
}
fn main() {

    setup_logging();
    info!("Starting!");
    warn!("No Config Found!!");
    println!("Hello, realm!");
    let challenge = auth_types::AUTH_LOGON_CHALLENGE_C{
        cmd: 1,
        error: 0,
        size: 0u16,
        gamename: 0,
        version1: 1,
        version2: 2,
        version3: 3,
        build: 34,
        platform: 1,
        os: 0,
        country: 1,
        timezone_bias: 0u32,
        ip: 1u32,
        I_len: 5u8,
        I: 5
    };
    println!("{:?}",challenge);
}
