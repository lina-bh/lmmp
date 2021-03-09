mod gl;
mod library;
mod widgets;
mod window;

use std::env;
use window::LmmpWindow;

fn main() {
    let library_path = String::from("~/Music");
    let library_path = library_path.replace("~", &env::var("HOME").unwrap());
    let files = library::index(&library_path).unwrap();

    let window = LmmpWindow::new();
    window.run();
}
