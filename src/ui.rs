// Steganography UI configuration and interfaces
// to Steganography class methods.

use crate::SETTINGS;
use crate::settings::Settings;
extern crate gtk;

use gtk::{gio, prelude::*};
use gtk::{Application, ApplicationWindow};

// Function to create application UI elements.
pub fn on_startup(app: &gtk::Application) {
    // Create an action for an 'About' menu item.
    // <TODO> Create a proper about box dialog on a Help top level menu.
    // On the same Help menu have a User Guide menu option.
        let about = gio::ActionEntry::builder("about")
        .activate(|_, _, _| println!("About was pressed"))
        .build();

    // Create an action for an applicatioin quit menu item.
    let quit = gio::ActionEntry::builder("quit")
        .activate(|app: &gtk::Application, _, _| app.quit())
        .build();

    // Add menu item actions to the application UI. 
    app.add_action_entries([about, quit]);

    // Create menubar selection items.
    // <TODO> Have top level menues for File and Edit.
    // On File menu have sub-menu items for Open and Save image functions.
    // On the edit menu have sub-menu items for Embed and Preview image functions.
    let menubar = {
        let file_menu = {
            let about_menu_item = gio::MenuItem::new(Some("About"), Some("app.about"));
            let quit_menu_item = gio::MenuItem::new(Some("Quit"), Some("app.quit"));

            let file_menu = gio::Menu::new();
            file_menu.append_item(&about_menu_item);
            file_menu.append_item(&quit_menu_item);
            file_menu
        };

        // Create an application menubar and associate items to it.
        let menubar = gio::Menu::new();
        menubar.append_submenu(Some("File"), &file_menu);
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
