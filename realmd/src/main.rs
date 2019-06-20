extern crate chrono;
extern crate futures;
#[macro_use] extern crate log;
#[macro_use] extern crate mysql;
extern crate simplelog;

mod auth_socket;
mod db;
mod logging;
fn main() {
    logging::setup_logging();
    let pool = db::get_pool();
    auth_socket::listen(pool.clone());
}
