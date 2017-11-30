use chrono::{Utc};
use simplelog::*;

use std::fs;
use std::fs::File;
/// checks if the logs directory exists or not
pub fn setup_logging() {
    match fs::read_dir("logs") {
        Ok(_) => init_logger(),
        Err(_) => create_log_dir()
    }
}
/// Meat and potatoes, actually sets up the loggers
fn init_logger() {
    let current_time = Utc::now();
    let log_file = format!("logs/realmd-{}.log", current_time.timestamp());
    CombinedLogger::init(vec![
        TermLogger::new(LogLevelFilter::Info, Config::default()).unwrap(),
        WriteLogger::new(LogLevelFilter::Info, Config::default(), File::create(&log_file).unwrap()),
    ]
    ).unwrap();
}

fn create_log_dir() {
    match fs::create_dir("logs") {
        Err(why) => println!("Couldn't create logging directory: {:?}", why.kind()),
        Ok(_) => {
            println!("Created logging directory!");
            init_logger();
        },
    }
}