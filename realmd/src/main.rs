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
   /* warn!("No Config Found!!");
    let mango_config = config::MangoConfig{
        database_string: "123:3306/db".to_string()
    };*/
    // warn!("Using default config: {:?}", &mangoConfig);

    socket::listen();

}
