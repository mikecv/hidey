// Steganography UI configuration and interfaces
// to Steganography class methods.

use log::{info};
use std::cell::RefCell;
use std::rc::Rc;

use gtk::gdk;
use gtk::glib;
extern crate gtk;
use gtk::{gio, prelude::*};
use gtk::{Application, ApplicationWindow};

use crate::SETTINGS;
use crate::settings::Settings;
use crate::steg::Steganography;

// Function to create application UI elements.
pub fn on_startup(app: &gtk::Application, img_steg: Rc<RefCell<Steganography>>) {
    // Create menubar.
    let menubar = gio::Menu::new();

    // Clone _img_steg to pass into menu item closures.
    let img_steg_clone = img_steg.clone();

    // Create an action for an 'Open' menu item.
    // Include reference to steg instance so that menu item
    // can trigger methods.
    let open = gio::ActionEntry::builder("open")
        .activate(move |_, _, _| open_image(&img_steg_clone))
        .build();

    // Create an action for a 'Save' menu item.
    let save = gio::ActionEntry::builder("save")
        .activate(|_, _, _| save_image())
        .build();

    // Create an action for an 'Embed' menu item.
    let embed = gio::ActionEntry::builder("embed")
        .activate(|_, _, _| embed_into_image())
        .build();

    // Create an action for a 'Preview' menu item.
    let preview = gio::ActionEntry::builder("preview")
        .activate(|_, _, _| preview_image())
        .build();

    // Create an action for an 'About' menu item.
    let about = gio::ActionEntry::builder("about")
        .activate(|_, _, _| about_app())
        .build();

    // Create an action for a 'Help' menu item.
    let help = gio::ActionEntry::builder("help")
        .activate(|_, _, _| help_app())
        .build();

    // Create an action for an applicatioin quit menu item.
    let quit = gio::ActionEntry::builder("quit")
        .activate(|app: &gtk::Application, _, _| app.quit())
        .build();

    // Add menu item actions to the application UI. 
    app.add_action_entries([open, save, embed, preview, about, help, quit]);

    // Create menubar full of menu options.
    let menubar = {
        let file_menu = {
            let open_menu_item = gio::MenuItem::new(Some("Open"), Some("app.open"));
            let save_menu_item = gio::MenuItem::new(Some("Save"), Some("app.save"));
            let quit_menu_item = gio::MenuItem::new(Some("Quit"), Some("app.quit"));

            let file_menu = gio::Menu::new();
            file_menu.append_item(&open_menu_item);
            file_menu.append_item(&save_menu_item);
            file_menu.append_item(&quit_menu_item);
            file_menu
        };

        let edit_menu = {
            let embed_menu_item = gio::MenuItem::new(Some("Embed"), Some("app.embed"));
            let preview_menu_item = gio::MenuItem::new(Some("Preview"), Some("app.preview"));

            let edit_menu = gio::Menu::new();
            edit_menu.append_item(&embed_menu_item);
            edit_menu.append_item(&preview_menu_item);
            edit_menu
        };

        let help_menu = {
            let about_menu_item = gio::MenuItem::new(Some("About"), Some("app.about"));
            let help_menu_item = gio::MenuItem::new(Some("Help"), Some("app.help"));

            let help_menu = gio::Menu::new();
            help_menu.append_item(&about_menu_item);
            help_menu.append_item(&help_menu_item);
            help_menu
        };
    
        // Create an application menubar and associate items to it.
        menubar.append_submenu(Some("File"), &file_menu);
        menubar.append_submenu(Some("Edit"), &edit_menu);
        menubar.append_submenu(Some("Help"), &help_menu);

        // Return menubar object.
        menubar
    };

    // Associate menubar with the application UI.
    app.set_menubar(Some(&menubar));
}

// Create the application window and add any children.
pub fn on_activate(application: &Application) {
    // Access lazy global settings from main.
    let settings_lock = SETTINGS.lock().unwrap();
    let settings: &Settings = &*settings_lock;

    // Create window container.
    let window = ApplicationWindow::builder()
        .application(application)
        .title("Hidey-Ho")
        .default_width(settings.window_width)
        .default_height(settings.window_height)
        .show_menubar(true)
        .build();

    // Present the UI and all elements.
    window.present();
}

pub fn open_image(_img_steg: &Rc<RefCell<Steganography>>) {
    info!("Open image menu item selected.");

    // Borrow the inner value mutably and obtain a mutable reference.
    // This will allow access to the fields and methods of Steganography.
    let mut img_steg_refmut = _img_steg.borrow_mut();
    let img_steg = &mut *img_steg_refmut;

    // <TODO> Remove these tests of access to Steganography variables and methods.
    img_steg.init_embed_params();
    img_steg.load_new_file("volleyballs-2.png".to_string());}

pub fn save_image() {
    info!("Save image menu item selected.");
}

pub fn embed_into_image() {
    info!("Embed into image menu item selected.");
}

pub fn preview_image() {
    info!("Preview image menu item selected.");
}

// Include the About logo file.
static LOGO_PNG: &[u8] = include_bytes!("../target/debug/static/hidey-ho.png");

pub fn about_app() {
    info!("About application menu item selected.");

    // Access lazy global settings from main.
    let settings_lock = SETTINGS.lock().unwrap();
    let settings: &Settings = &*settings_lock;

    let bytes = glib::Bytes::from_static(LOGO_PNG);
    let logo = gdk::Texture::from_bytes(&bytes).expect("Failed to load About logo");
    let dialog = gtk::AboutDialog::builder()
        .program_name(settings.program_name.clone())
        .version(settings.program_ver.clone())
        .license_type(gtk::License::Lgpl30)
        .authors(settings.program_devs.clone())
        .logo(&logo)
        .build();

    dialog.set_modal(false);
    dialog.present();
}

pub fn help_app() {
    info!("Application help menu item selected.");
}
