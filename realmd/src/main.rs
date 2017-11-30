#[macro_use] extern crate log;
extern crate simplelog;
extern crate chrono;
extern crate futures;
#[macro_use] extern crate tokio_core;
extern crate tokio_io;
mod auth_types;
mod logging;
mod config;
mod socket;
fn main() {
    info!("Hello, realm!");

    logging::setup_logging();
    warn!("No Config Found!!");
    let mango_config = config::MangoConfig{
        database_string: "123:3306/db".to_string()
    };
    // warn!("Using default config: {:?}", &mangoConfig);
    /*let challenge = auth_types::AUTH_LOGON_CHALLENGE_C{
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
    println!("Test Challenge: {:?}",challenge);*/
    socket::listen();

}
