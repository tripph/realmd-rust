#[macro_use] extern crate log;
extern crate simplelog;
extern crate chrono;
mod auth_types;
mod logging;

fn main() {

    logging::setup_logging();
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
    println!("Test Challenge: {:?}",challenge);
}
