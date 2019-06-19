
use mysql as my;
use simplelog::LogLevel::*;

pub fn get_pool() -> my::Pool {
    info!("Opening DB Pool!");
    return mysql::Pool::new("mysql://root:root@localhost:3306/realmd").unwrap();
}

