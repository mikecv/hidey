// Steganography application.

use log::info;
use log4rs;
use std::fs::File;
use std::io::prelude::*;
use serde_yaml;

use crate::settings::Settings;
use crate::steg::Steganography;

pub mod settings;
pub mod steg;

fn main() {
    // Set up application logging.
    // Configuration in log4rs.yml .
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();

    // Get application metadata to include in initial logging.
    info!("Application started, version: {}", env!("CARGO_PKG_VERSION"));

    // Read YAML file
    let mut file = File::open("settings.yml").expect("Unable to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read file");

    // Deserialize YAML into Settings struct
    let settings: Settings = serde_yaml::from_str(&contents).expect("Unable to parse YAML");
    info!("Application settings loaded, version: {}", settings.settings_version);

    // Instatiate a steganography struct.
    let mut img_steg = Steganography::init(settings);

    // <TEMP>
    // Testing Steganography struct and methods.
 
    // <Test 1>
    // Loading a file and checking it for embedded files.
    // img_steg.init_embed_params();
    // img_steg.load_new_file("volleyballs.png".to_string());

    // <Test 2a>
    // Loading a file and and embed file(s) into it.
    // img_steg.init_embed_params();
    // img_steg.load_new_file("rat.png".to_string());
    // // Embed files into image.
    // let embed_files = vec!["/home/mike/hidey/images/kitten-grass.png", "/home/mike/hidey/images/kitten-in-basket.jpg"];
    // if let Err(err) = img_steg.embed_files(&embed_files) {
    //     eprintln!("Error: {}", err);
    // }

    // <Test 2b>
    // Load the new image with embedded file and extract embedded file.
    img_steg.init_embed_params();
    img_steg.load_new_file("rat.png".to_string());
}
