// Steganography UI configuration and interfaces
// to Steganography class methods.

extern crate gio;
extern crate gtk;

use gtk::prelude::*;
use gtk::{Button};
use gtk::{Application, ApplicationWindow};

pub fn build_ui(app: &Application, window_width: i32, window_height: i32) {
    let v_box = gtk::Box::new(gtk::Orientation::Vertical, 10);

    // Call the function to create the test button and add it to the v_box.
    let test_button = create_test_button();
    v_box.append(&test_button);

    // Create the application window and add any children.
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Hidey-Ho")
        .default_width(window_width)
        .default_height(window_height)
        .child(&v_box)
        .build();

    window.present();
}

// Function to create a test button.
// Not part of the Steganography application.
fn create_test_button() -> Button {
    let test_button = Button::builder()
        .label("Test Button, Press me!")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    // Connect to "clicked" signal of test button.
    test_button.connect_clicked(|test_button| {
        test_button.set_label("Test button pressed!");
    });

    // Return the test button.
    test_button
}
