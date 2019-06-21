extern crate chrono;
extern crate core;
extern crate futures;
#[macro_use]
extern crate log;
#[macro_use]
extern crate mysql;
extern crate rand;
extern crate simplelog;
extern crate srp;

mod auth_socket;
mod db;
mod logging;
fn main() {
    logging::setup_logging();
    let pool = db::get_pool();
    auth_socket::listen(pool.clone());
}
