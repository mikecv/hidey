// Steganography application.

use log::info;
use log4rs;
use gtk::prelude::*;
use lazy_static::lazy_static;
use std::cell::RefCell;
use std::rc::Rc;
use std::fs::File;
use std::io::prelude::*;
use std::sync::Mutex;
use serde_yaml;

use crate::settings::Settings;
use crate::steg::Steganography;
use crate::ui::{on_activate, on_startup};

pub mod settings;
pub mod steg;
mod ui;

// Create a global variable for applications settings.
// This was available in other files.
lazy_static! {
    static ref SETTINGS: Mutex<Settings> = {
        // Read YAML settings file.
        let mut file = File::open("settings.yml").expect("Unable to open file");
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("Unable to read file");

        // Deserialize YAML into Settings struct.
        let settings: Settings = serde_yaml::from_str(&contents).expect("Unable to parse YAML");
        info!("Application settings loaded, version: {}", settings.settings_version);
        Mutex::new(settings)
    };
}

// Steganoraphy mainline.
fn main() {
    // Set up application logging.
    // Configuration held in log4rs.yml .
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();

    // Get application metadata to include in initial logging.
    info!("Application started, version: {}", env!("CARGO_PKG_VERSION"));

    // Instatiate a steganography struct.
    // Make interior shareable as need to pass to UI
    // menu servicing functions.
    let img_steg = Rc::new(RefCell::new(Steganography::init()));

    // Create a new GTK application.
    let application = gtk::Application::builder()
    .application_id("com.example.steganography")
    .build();

    // Start up and activate UI application.
    application.connect_startup(move |app| on_startup(app, img_steg.clone()));
    application.connect_activate(on_activate);
    application.run();
}
