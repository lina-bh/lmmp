mod ffmpeg;
mod mp3;
mod mp4;

use std::convert::identity;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub struct Library {
    tracks: Vec<Track>,
}

#[derive(Debug)]
pub struct Track {
    pub path: PathBuf,
    pub album_artist: String,
    pub album: String,
    pub track_no: u32,
    pub title: String,
}

impl Track {
    pub fn new(path: impl AsRef<Path>) -> anyhow::Result<Option<Track>> {
        let p = path.as_ref();
        Ok(if let Some(ext) = p.extension().and_then(OsStr::to_str) {
            let parse = match ext {
                "mp3" => mp3::parse,
                "m4a" => mp4::parse,
                "ogg" | "flac" | "opus" => ffmpeg::parse,
                _ => return Ok(None),
            };
            parse(&p)?
        } else {
            None
        })
    }
}

impl Library {
    pub fn index(path: impl AsRef<Path>) -> anyhow::Result<Library> {
        let ents = WalkDir::new(path)
            .into_iter()
            .collect::<Result<Vec<walkdir::DirEntry>, walkdir::Error>>()?;

        let paths = ents
            .iter()
            .filter(|e| !e.file_type().is_dir())
            .map(|e| e.path());

        let tracks = paths
            .map(|p| Track::new(&p))
            .collect::<Result<Vec<Option<Track>>, anyhow::Error>>()?
            .into_iter()
            .filter_map(identity)
            .collect::<Vec<Track>>();

        Ok(Library { tracks })
    }
}
