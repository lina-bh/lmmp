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
    let hardcoded = "~/Music".replace("~", &env::var("HOME").unwrap());

    PathBuf::from(hardcoded)
}

fn main() {
    set_ffmpeg_loglevel();

    use lexical_sort::{lexical_cmp, StringSort};

    let lib = library::Library::index(&get_library_path()).unwrap();
    let mut artists = lib.artists().collect::<Vec<&str>>();
    artists.string_sort_unstable(lexical_cmp);
    for artist in artists {
        println!("{}", artist);
        if let Some(albums) = lib.albums(artist) {
            for album in albums {
                println!("- {}", album);
                for track in lib.album(artist, album) {
                    println!("  {:2}. {} ({:?})", track.track_no, track.title, track.path);
                }
            }
        }
    }

    #[cfg(feature = "ui")]
    {
        let window = window::LmmpWindow::new();
        window.run();
    }
}
