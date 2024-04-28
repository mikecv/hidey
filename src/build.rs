use std::fs;

fn main() {
    // Copy the static folder and all its contents to the target directory.
    fs::copy("static", "./target/debug/static").expect("Failed to copy static folder");
    fs::copy("static", "./target/release/static").expect("Failed to copy static folder");
}
