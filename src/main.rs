use std::fs;
use std::path::PathBuf;

use sciter;

fn main() {
    let mut frame = sciter::Window::new();
    let html_file_path = PathBuf::from("./app.html");
    let html_file = fs::canonicalize(&html_file_path).unwrap();

    frame.load_file(html_file.to_str().unwrap());
    frame.run_app();
}
