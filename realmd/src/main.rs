#[macro_use] extern crate mysql;
#[macro_use] extern crate log;
extern crate simplelog;
extern crate chrono;

mod auth_socket;
mod db;
mod logging;
fn main() {
    logging::setup_logging();
    let pool = db::get_pool();
    auth_socket::listen(pool.clone());
}
