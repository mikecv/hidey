// Steganography application

use log::info;
use log4rs;

fn main() {
    // Set up application logging.
    // Configuration in log4rs.yml
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();

    // Get application metadata to include in initial logging.
    info!("Application started, version: {}", env!("CARGO_PKG_VERSION"));
}
