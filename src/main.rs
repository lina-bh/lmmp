mod library;
#[cfg(feature = "ui")]
mod window;

use ffmpeg_next as ffmpeg;
use std::env;
use std::path::PathBuf;

fn set_ffmpeg_loglevel() {
    use ffmpeg::util::log;

    log::set_level(log::Level::Error);
}

fn get_library_path() -> PathBuf {
    let mut args = env::args();
    let p = match args.nth(1) {
        Some(p) => p,
        // "~/Music".replace("~", &env::var("HOME").unwrap()),
        None => format!("{}/Music", env::var("HOME").expect("$HOME")),
    };

    PathBuf::from(p)
}

fn main() {
    set_ffmpeg_loglevel();

    let lib = library::Library::index(&get_library_path()).unwrap();

    #[cfg(feature = "ui")]
    {
        let window = window::LmmpWindow::new();
        window.run();
    }
}
