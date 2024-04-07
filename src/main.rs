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
    // Configuration held in log4rs.yml .
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();

    // Get application metadata to include in initial logging.
    info!("Application started, version: {}", env!("CARGO_PKG_VERSION"));

    // Read YAML settings file.
    let mut file = File::open("settings.yml").expect("Unable to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read file");

    // Deserialize YAML into Settings struct.
    let settings: Settings = serde_yaml::from_str(&contents).expect("Unable to parse YAML");
    info!("Application settings loaded, version: {}", settings.settings_version);

    // Instatiate a steganography struct.
    let mut img_steg = Steganography::init(settings);

    // <TEMP>
    // Testing Steganography struct and methods.
 
    // <Test 1>
    // Loading a file and checking it for embedded files.
    img_steg.init_embed_params();
    img_steg.load_new_file("volleyballs-2.png".to_string());
    // img_steg.load_new_file("rat.png".to_string());

    // <Test 2a>
    // Loading a file and embed file(s) into it.
    // img_steg.init_embed_params();
    // img_steg.load_new_file("volleyballs-2.png".to_string());

    // Embed files into image.
    // Varients to embed with or without password.
    // let files_to_embed = vec!["/home/mike/hidey/images/kitten-grass.png", "/home/mike/hidey/images/kitten-in-basket.jpg"];

    // Embed with no password.
    // if let Err(err) = img_steg.embed_files(false, "", &files_to_embed) {
    // Embed with a password.
    // if let Err(err) = img_steg.embed_files(true, "ratpig", &files_to_embed) {
    //     eprintln!("Error: {}", err);
    // }
    // else {
        // Save the embedded image to a file.
        // Versions for empty filename meaning overwrite, or new file.
        // For a new file specify empty string for file path/name.
        // img_steg.save_image("".to_string())
    //     img_steg.save_image("./images/rat.png".to_string())
    // }

    // <Test 2b>
    // Load the new image with embedded data and extract the data.
    // First get to the section of the embedded data that indicates
    // if a password is required (or not).
    // Extracting the actual data is a separate method regardless
    // of whether there is a password or not.

    // Load the file.
    // This will indicate if it embedded and if is password protected.
    // img_steg.init_embed_params();
    // img_steg.load_new_file("volleyballs-2.png".to_string());
    // img_steg.load_new_file("rat.png".to_string());

    // Extracted the embedded data.
    // There may be a password required.
    // If there is no password a blank string is still sent.
    // Test for version with and without passwords.
    // Test with right and wrong passwords.

    // img_steg.extract_data("".to_string());
    // img_steg.extract_data("pigdog".to_string());
    // img_steg.extract_data("ratpig".to_string());
}
