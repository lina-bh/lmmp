mod library;
#[cfg(feature = "ui")]
mod window;

use ffmpeg_next as ffmpeg;
use std::env;

fn set_ffmpeg_loglevel() {
    use ffmpeg::util::log;

    log::set_level(log::Level::Error);
}

fn main() {
    set_ffmpeg_loglevel();

    // return;
    let library_path = String::from("~/Music");
    let library_path = library_path.replace("~", &env::var("HOME").unwrap());
    let files = library::index(&library_path).unwrap();
    // for f in files {
    //     println!("{:?}", f);
    // }
    return;

    // library::vorb::_test();

    #[cfg(feature = "ui")]
    {
        let window = window::LmmpWindow::new();
        window.run();
    }
}
