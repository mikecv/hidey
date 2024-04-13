// Steganography UI configuration and interfaces
// to Steganography class methods.

use gtk::prelude::*;
use gtk::{Button};

pub fn build_ui(app: &gtk::Application, window_width: i32, window_height: i32) {

    // Create a test button with label and margins.
    let test_button = Button::builder()
        .label("Press me!")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    // Connect to "clicked" signal of `button`.
    test_button.connect_clicked(|test_button| {
        // Set the label to "Hello World!" after the button has been pressed.
        test_button.set_label("Hello World!");
    });

    // Create a new application window.
    let window = gtk::ApplicationWindow::builder()
        .application(app)
        .title("Hidey-Ho")
        .default_width(window_width)
        .default_height(window_height)
        // Attach the test button to the window.
        .child(&test_button)
        .build();

    window.present();
}
