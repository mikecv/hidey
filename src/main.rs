// Steganography application.

use log::info;
use log4rs;

use gtk::prelude::*;

use lazy_static::lazy_static;
use std::fs::File;
use std::io::prelude::*;
use std::sync::Mutex;
use serde_yaml;

use crate::settings::Settings;
use crate::steg::Steganography;
use crate::ui::build_ui;

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

fn main() {
    // Set up application logging.
    // Configuration held in log4rs.yml .
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();

    // Get application metadata to include in initial logging.
    info!("Application started, version: {}", env!("CARGO_PKG_VERSION"));

    // Instatiate a steganography struct.
    let mut _img_steg = Steganography::init();

    // Create a new GTK application.
    let app = gtk::Application::builder()
    .application_id("com.example.steganography")
    .build();

    // Build Steganography UI.
    app.connect_activate(|app| {
        let settings = SETTINGS.lock().unwrap();
        let window_width = settings.window_width;
        let window_height = settings.window_height;

        build_ui(app, window_width, window_height);
    });

    app.run();
}
